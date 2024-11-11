use std::collections::HashMap;

use crate::expense::Expense;
use crate::rounding::Round;
use crate::upserting::Upsertable;
use crate::user::User;

#[derive(Debug, PartialEq)]
pub struct SplitError;

#[derive(Debug, PartialEq)]
pub struct Split<'a> {
    expenses: Vec<&'a Expense<'a>>,
    user_subtotals: HashMap<&'a User, f32>,
    user_taxes: HashMap<&'a User, f32>,
    user_tips: HashMap<&'a User, f32>,
    user_totals: HashMap<&'a User, f32>,
    subtotal: f32,
    tax: f32,
    tip: f32,
    total: f32,
}

impl<'a> Split<'a> {
    pub fn new(expenses: Vec<&'a Expense<'a>>, tax: f32, tip: f32) -> Result<Self, SplitError> {
        if expenses.len() == 0 {
            return Err(SplitError);
        }

        let mut subtotal: f32 = 0_f32;
        for expense in &expenses {
            subtotal += expense.get_amount();
        }

        if subtotal < 0_f32 {
            return Err(SplitError);
        }

        if tax < 0_f32 {
            return Err(SplitError);
        }

        if tip < 0_f32 {
            return Err(SplitError);
        }

        Ok(Split {
            expenses,
            user_subtotals: HashMap::new(),
            user_taxes: HashMap::new(),
            user_tips: HashMap::new(),
            user_totals: HashMap::new(),
            subtotal,
            tax,
            tip,
            total: subtotal + tax + tip,
        })
    }

    fn calculate_subtotals(&mut self) {
        for expense in &self.expenses {
            self.user_subtotals
                .upsert_all(&expense.get_user_to_amount());
        }
    }

    fn calculate_taxes_and_tips(&mut self) {
        let mut remaining_tax: f32 = self.tax;
        let mut remaining_tip: f32 = self.tip;

        for (k, v) in &self.user_subtotals {
            let percent: f32 = v / self.subtotal;

            let tax: f32 = (self.tax * percent).to_two_decimals();
            let tip: f32 = (self.tip * percent).to_two_decimals();

            if tax < remaining_tax {
                self.user_taxes.insert(k, tax);
            } else {
                self.user_taxes.insert(k, remaining_tax);
            }

            if tip < remaining_tip {
                self.user_tips.insert(k, tip);
            } else {
                self.user_tips.insert(k, remaining_tip);
            }

            remaining_tax = (remaining_tax - tax).to_two_decimals();
            remaining_tip = (remaining_tip - tip).to_two_decimals();
        }
    }

    fn calculate_totals(&mut self) {
        self.user_totals.upsert_all(&self.user_subtotals);
        self.user_totals.upsert_all(&self.user_taxes);
        self.user_totals.upsert_all(&self.user_tips);
    }

    pub fn process(&mut self) -> HashMap<&User, f32> {
        self.calculate_subtotals();
        self.calculate_taxes_and_tips();
        self.calculate_totals();

        self.user_totals.clone()
    }

    pub fn print(&self) {
        println!();
        for (k, v) in &self.user_totals {
            let subtotal: &f32 = self.user_subtotals.get(k).unwrap();
            let tax: &f32 = self.user_taxes.get(k).unwrap();
            let tip: &f32 = self.user_tips.get(k).unwrap();
            println!(
                "{} - subtotal: ${}, tax: ${}, tip: ${}, total: ${}",
                k, subtotal, tax, tip, v
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expense::PendingExpense;

    use super::*;

    #[test]
    fn test_new_success() {
        let user_one: User = User::new("user_one").unwrap();
        let user_two: User = User::new("user_two").unwrap();

        let mut pending_expense_one: PendingExpense =
            PendingExpense::new("pending_expense_one", 5.50).unwrap();
        let mut pending_expense_two: PendingExpense =
            PendingExpense::new("pending_expense_two", 7.25).unwrap();

        pending_expense_one.add_participant(&user_one);
        pending_expense_one.add_participant(&user_two);
        let expense_one: Expense = pending_expense_one.finalize().unwrap();

        pending_expense_two.add_participant(&user_two);
        let expense_two: Expense = pending_expense_two.finalize().unwrap();

        let expenses: Vec<&Expense> = vec![&expense_one, &expense_two];
        let tax: f32 = 2.20;
        let tip: f32 = 3.45;
        let subtotal: f32 = expenses.iter().map(|e| e.get_amount()).sum();

        let maybe_split: Result<Split, SplitError> = Split::new(expenses, tax, tip);

        let expected: Split = Split {
            expenses: vec![&expense_one, &expense_two],
            user_subtotals: HashMap::new(),
            user_taxes: HashMap::new(),
            user_tips: HashMap::new(),
            user_totals: HashMap::new(),
            subtotal,
            tax,
            tip,
            total: subtotal + tax + tip,
        };

        assert!(maybe_split.is_ok());
        let split: Split = maybe_split.unwrap();
        assert_eq!(split, expected);
    }

    #[test]
    fn test_new_failure_expenses_empty() {
        let tax: f32 = 2.20;
        let tip: f32 = 3.45;

        let maybe_split: Result<Split, SplitError> = Split::new(Vec::new(), tax, tip);
        assert!(maybe_split.is_err());
        assert_eq!(maybe_split.unwrap_err(), SplitError);
    }

    #[test]
    fn test_new_failure_tip_is_negative() {
        let user_one: User = User::new("user_one").unwrap();
        let user_two: User = User::new("user_two").unwrap();

        let mut pending_expense_one: PendingExpense =
            PendingExpense::new("pending_expense_one", 5.50).unwrap();
        let mut pending_expense_two: PendingExpense =
            PendingExpense::new("pending_expense_two", 7.25).unwrap();

        pending_expense_one.add_participant(&user_one);
        pending_expense_one.add_participant(&user_two);
        let expense_one: Expense = pending_expense_one.finalize().unwrap();

        pending_expense_two.add_participant(&user_two);
        let expense_two: Expense = pending_expense_two.finalize().unwrap();

        let expenses: Vec<&Expense> = vec![&expense_one, &expense_two];
        let tax: f32 = -2.20;
        let tip: f32 = 3.45;

        let maybe_split: Result<Split, SplitError> = Split::new(expenses, tax, tip);
        assert!(maybe_split.is_err());
        assert_eq!(maybe_split.unwrap_err(), SplitError);
    }

    #[test]
    fn test_new_failure_tax_is_negative() {
        let user_one: User = User::new("user_one").unwrap();
        let user_two: User = User::new("user_two").unwrap();

        let mut pending_expense_one: PendingExpense =
            PendingExpense::new("pending_expense_one", 5.50).unwrap();
        let mut pending_expense_two: PendingExpense =
            PendingExpense::new("pending_expense_two", 7.25).unwrap();

        pending_expense_one.add_participant(&user_one);
        pending_expense_one.add_participant(&user_two);
        let expense_one: Expense = pending_expense_one.finalize().unwrap();

        pending_expense_two.add_participant(&user_two);
        let expense_two: Expense = pending_expense_two.finalize().unwrap();

        let expenses: Vec<&Expense> = vec![&expense_one, &expense_two];
        let tax: f32 = 2.20;
        let tip: f32 = -3.45;

        let maybe_split: Result<Split, SplitError> = Split::new(expenses, tax, tip);
        assert!(maybe_split.is_err());
        assert_eq!(maybe_split.unwrap_err(), SplitError);
    }

    #[test]
    fn test_process() {
        let user_one: User = User::new("user_one").unwrap();
        let user_two: User = User::new("user_two").unwrap();

        let mut pending_expense_one: PendingExpense =
            PendingExpense::new("pending_expense_one", 5.50).unwrap();
        let mut pending_expense_two: PendingExpense =
            PendingExpense::new("pending_expense_two", 7.25).unwrap();

        pending_expense_one.add_participant(&user_one);
        pending_expense_one.add_participant(&user_two);
        let mut expense_one: Expense = pending_expense_one.finalize().unwrap();
        expense_one.calculate();

        pending_expense_two.add_participant(&user_two);
        let mut expense_two: Expense = pending_expense_two.finalize().unwrap();
        expense_two.calculate();

        let expenses: Vec<&Expense> = vec![&expense_one, &expense_two];
        let subtotal: f32 = 12.75;
        let tax: f32 = 2.20;
        let tip: f32 = 3.45;

        let mut split: Split = Split::new(expenses, tax, tip).unwrap();
        _ = split.process();

        let mut user_subtotals: HashMap<&User, f32> = HashMap::new();
        user_subtotals.insert(&user_one, 2.75);
        user_subtotals.insert(&user_two, 10.00);

        let mut user_taxes: HashMap<&User, f32> = HashMap::new();
        user_taxes.insert(&user_one, 0.47);
        user_taxes.insert(&user_two, 1.73);

        let mut user_tips: HashMap<&User, f32> = HashMap::new();
        user_tips.insert(&user_one, 0.74);
        user_tips.insert(&user_two, 2.71);

        let mut user_totals: HashMap<&User, f32> = HashMap::new();
        user_totals.insert(&user_one, 3.96);
        user_totals.insert(&user_two, 14.44);

        let expected: Split = Split {
            expenses: vec![&expense_one, &expense_two],
            user_subtotals,
            user_taxes,
            user_tips,
            user_totals,
            subtotal,
            tax,
            tip,
            total: subtotal + tax + tip,
        };

        assert_eq!(split, expected);
    }
}
