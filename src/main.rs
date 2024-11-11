use std::collections::HashMap;
use std::io;

use heesab::expense::Expense;
use heesab::expense::PendingExpense;
use heesab::expense::PendingExpenseError;
use heesab::split::Split;
use heesab::split::SplitError;
use heesab::user::User;
use heesab::user::UserError;

fn main() {
    let mut name_to_user: HashMap<String, User> = HashMap::new();
    let mut valid_participants: bool = false;
    while !valid_participants {
        name_to_user.clear();
        let mut participants: String = String::new();
        println!(
            "Enter all unique participant names in the following format: <participant one name>:<participant two name>:..."
        );

        io::stdin()
            .read_line(&mut participants)
            .expect("Failed to read line");

        valid_participants = true;
        for name in participants.trim().split(':') {
            let maybe_user: Result<User, UserError> = User::new(name.trim());
            if maybe_user.is_err() {
                println!("Error: Name is invalid. Please try again.");
                valid_participants = false;
            } else if name_to_user.contains_key(name) {
                println!("Error: Cannot use duplicate name. Please try again.");
                valid_participants = false;
            } else {
                name_to_user.insert(name.to_string(), maybe_user.unwrap());
            }
        }
    }

    let mut expenses: Vec<Expense> = Vec::new();
    loop {
        let mut expense_input: String = String::new();
        println!("\nEnter an expense in the following format: <expense name>:<amount>. Type done to move to next step.");

        io::stdin()
            .read_line(&mut expense_input)
            .expect("Failed to read line");

        if expense_input.trim() == "done".to_string() {
            break;
        }

        let expense_details: Vec<&str> = expense_input.trim().split(':').collect();
        if expense_details.len() != 2 {
            println!("Error: invalid format. Please try again.");
            continue;
        }

        let maybe_amount: Result<f32, _> = expense_details[1].parse::<f32>();

        if maybe_amount.is_err() {
            println!("Error: invalid format. Please try again.");
            continue;
        }

        let maybe_pending_expense: Result<PendingExpense, PendingExpenseError> =
            PendingExpense::new(expense_details[0], maybe_amount.unwrap());

        if maybe_pending_expense.is_err() {
            println!("Error: Expense details are invalid. Please try again.");
            continue;
        }

        let mut pending_expense: PendingExpense = maybe_pending_expense.unwrap();

        let mut expense_participants: String = String::new();
        println!();
        for name in name_to_user.keys() {
            println!("{}", name);
        }
        println!(
            "Enter participant names from above for this expense in the following format: <participant one name>:<participant two name>:..."
        );

        io::stdin()
            .read_line(&mut expense_participants)
            .expect("Failed to read line");

        for name in expense_participants.trim().split(':') {
            match name_to_user.get(name) {
                None => {
                    println!(
                        "Error: {} did not match existing participant. Exiting",
                        name
                    );
                }
                Some(user) => {
                    pending_expense.add_participant(user);
                }
            }
        }
        let mut expense: Expense = pending_expense
            .finalize()
            .expect("Error: No valid participants. Exiting.");
        expense.calculate();
        expenses.push(expense);
    }

    let mut calc_input: String = String::new();
    let mut calcs: Vec<&str>;

    loop {
        println!("\nEnter tax and tip info in the following format: <tax>:<tip>");
        io::stdin()
            .read_line(&mut calc_input)
            .expect("Failed to read line");

        calcs = calc_input.trim().split(':').collect();

        if calcs.len() != 2 {
            println!("Error: invalid format. Please try again.");
            continue;
        }
        let maybe_split: Result<Split, SplitError> = Split::new(
            expenses.iter().map(|e| e).collect(),
            calcs[0].parse::<f32>().expect("Error: Could not parse."),
            calcs[1].parse::<f32>().expect("Error: Could not parse."),
        );

        if maybe_split.is_err() {
            println!("Error: invalid tax or tip details. Please try again.");
            continue;
        }

        let mut split: Split = maybe_split.unwrap();

        split.process();
        split.print();
        break;
    }
}
