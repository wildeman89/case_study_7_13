pub mod bank;
use crate::bank::account::Account;
use std::io::{self, Write};

fn main() {
    let mut account = Account::new();
    let mut choice: String = "".to_string();
    while !choice.eq("7") {
        display_menu();
        choice = get_input("Enter Choice: ");
        choice = choice.trim().to_string();

        match choice.as_str() {
            "1" => println!("Account Balance is: {:.2}", account.get_balance()),
            "2" => println!("Number of transactions: {}", account.get_transactions()),
            "3" => println!(
                "Interest earned for this Period: {:.2}",
                account.get_interest()
            ),
            "4" => make_deposit(&mut account),
            "5" => make_withdrawal(&mut account),
            "6" => println!("Add Interest::TODO"),
            "7" => println!("Thank you. Have a good day!"),
            _ => println!("Must be a value between 1 and 7. Try again"),
        };
    }
}

fn display_menu() {
    println!("\n{:^80}", "MENU");
    println!("\n1) Display Account Balance");
    println!("2) Display Number of Transactions");
    println!("3) Display Interest Earned for Current Period");
    println!("4) Make a Deposit");
    println!("5) Make a Withdrawal");
    println!("6) Add Interest for Current Period");
    println!("7) Exit Program");
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn make_deposit(acnt: &mut Account) {
    let input = get_input("Enter the amount for deposit: ");
    let input = input
        .trim()
        .parse::<f64>()
        .expect("Failed to convert to f64 -- deposit");
    acnt.make_deposit(input);
}

fn make_withdrawal(acnt: &mut Account) {
    let input = get_input("Enter the amount of the withdrawl: ");
    let input: f64 = input
        .trim()
        .parse::<f64>()
        .expect("failed to convert to f64 -- withdrawal");
    let result = acnt.withdraw(input);
    match result {
        Some(_) => println!("Withdrawl Accepted."),
        None => println!("Could not process the withdrawal."),
    };
}
