use std::collections::HashMap;

use crate::rounding::Round;
use crate::user::User;

#[derive(Debug, PartialEq)]
pub struct PendingExpenseError;

#[derive(Debug)]
pub struct PendingExpense<'a> {
    name: String,
    amount: f32,
    participants: Vec<&'a User>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expense<'a> {
    name: String,
    amount: f32,
    participants: Vec<&'a User>,
    num_participants: u32,
    user_to_amount: HashMap<&'a User, f32>,
}

impl<'a> PendingExpense<'a> {
    pub fn new(name: &str, amount: f32) -> Result<Self, PendingExpenseError> {
        if name.len() == 0 {
            return Err(PendingExpenseError);
        }

        if amount < 0_f32 {
            return Err(PendingExpenseError);
        }

        Ok(PendingExpense {
            name: name.to_string(),
            amount,
            participants: Vec::new(),
        })
    }

    pub fn add_participant(&mut self, user: &'a User) {
        self.participants.push(user);
    }

    pub fn finalize(self) -> Result<Expense<'a>, PendingExpenseError> {
        if self.participants.len() == 0 {
            return Err(PendingExpenseError);
        }

        let num_participants: u32 = self.participants.len() as u32;
        Ok(Expense {
            name: self.name,
            amount: self.amount,
            participants: self.participants,
            num_participants,
            user_to_amount: HashMap::new(),
        })
    }
}

impl<'a> Expense<'a> {
    pub fn calculate(&mut self) {
        let per_user_amount_unrounded: f32 = self.amount / self.num_participants as f32;
        let per_user_amount: f32 = per_user_amount_unrounded.to_two_decimals();

        let mut remaining: f32 = self.amount;

        for participant in &self.participants {
            if per_user_amount < remaining {
                self.user_to_amount.insert(participant, per_user_amount);
            } else {
                self.user_to_amount.insert(participant, remaining);
            }
            remaining = (remaining - per_user_amount).to_two_decimals();
        }
    }

    pub fn get_user_to_amount(&self) -> HashMap<&User, f32> {
        self.user_to_amount.clone()
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_pending_expense {
        use super::*;

        #[test]
        fn test_pending_expense_new_success() {
            let name: &str = "random";
            let amount: f32 = 12.34;

            let maybe_pending_expense: Result<PendingExpense, PendingExpenseError> =
                PendingExpense::new(name, amount);

            assert!(maybe_pending_expense.is_ok());
            let pending_expense: PendingExpense = maybe_pending_expense.unwrap();
            assert_eq!(pending_expense.name, name);
            assert_eq!(pending_expense.amount, amount);
        }

        #[test]
        fn test_pending_expense_new_failure_name_is_empty() {
            let name: &str = "";
            let amount: f32 = 12.34;

            let maybe_pending_expense: Result<PendingExpense, PendingExpenseError> =
                PendingExpense::new(name, amount);

            assert!(maybe_pending_expense.is_err());
            assert_eq!(maybe_pending_expense.unwrap_err(), PendingExpenseError);
        }

        #[test]
        fn test_pending_expense_new_failure_amount_is_negative() {
            let name: &str = "random";
            let amount: f32 = -12.34;

            let maybe_pending_expense: Result<PendingExpense, PendingExpenseError> =
                PendingExpense::new(name, amount);

            assert!(maybe_pending_expense.is_err());
            assert_eq!(maybe_pending_expense.unwrap_err(), PendingExpenseError);
        }

        #[test]
        fn test_pending_expense_add_participant() {
            let mut pending_expense: PendingExpense = PendingExpense::new("random", 12.34).unwrap();
            let user_one: User = User::new("user_one").unwrap();
            let user_two: User = User::new("user_two").unwrap();

            pending_expense.add_participant(&user_one);
            assert_eq!(pending_expense.participants.len(), 1);

            pending_expense.add_participant(&user_two);
            assert_eq!(pending_expense.participants.len(), 2);
        }

        #[test]
        fn test_pending_expense_finalize_success() {
            let mut pending_expense: PendingExpense = PendingExpense::new("random", 12.34).unwrap();
            let user_one: User = User::new("user_one").unwrap();
            let user_two: User = User::new("user_two").unwrap();
            pending_expense.add_participant(&user_one);
            pending_expense.add_participant(&user_two);

            let maybe_expense: Result<Expense, PendingExpenseError> = pending_expense.finalize();
            assert!(maybe_expense.is_ok());
            let expense: Expense = maybe_expense.unwrap();

            let expected_expense: Expense = Expense {
                name: "random".to_string(),
                amount: 12.34,
                participants: vec![&user_one, &user_two],
                num_participants: 2,
                user_to_amount: HashMap::new(),
            };

            assert_eq!(expense, expected_expense)
        }

        #[test]
        fn test_pending_expense_finalize_failure_no_participants() {
            let pending_expense: PendingExpense = PendingExpense::new("random", 12.34).unwrap();

            let maybe_expense: Result<Expense, PendingExpenseError> = pending_expense.finalize();
            assert!(maybe_expense.is_err());
            assert_eq!(maybe_expense.unwrap_err(), PendingExpenseError)
        }
    }

    mod test_expense {
        use super::*;

        #[test]
        fn test_calculate() {
            let mut pending_expense: PendingExpense = PendingExpense::new("random", 12.50).unwrap();
            let user_one: User = User::new("user_one").unwrap();
            let user_two: User = User::new("user_two").unwrap();
            pending_expense.add_participant(&user_one);
            pending_expense.add_participant(&user_two);

            let mut expense: Expense = pending_expense.finalize().unwrap();
            expense.calculate();

            let mut expected: HashMap<&User, f32> = HashMap::new();
            expected.insert(&user_one, 6.25);
            expected.insert(&user_two, 6.25);

            assert_eq!(expense.user_to_amount, expected);
        }
    }
}
