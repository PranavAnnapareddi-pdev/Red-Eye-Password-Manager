use std::env;
use crate::password_manager::PasswordEntry;

pub fn parse_arguments() {
    let args: Vec<String> = env::args().collect();

    // Print out the arguments (for now, just to test)
    if args.len() < 2 {
        println!("Usage: password_manager <command> [arguments]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 5 {
                println!("Usage: password_manager add <account> <username> <password>");
            } else {
                let account = args[2].clone();
                let username = args[3].clone();
                let password = args[4].clone();
                PasswordEntry::add(account, username, password);
                println!("Password added successfully!");
            }
        },
        "retrieve" => {
            if args.len() < 3 {
                println!("Usage: password_manager retrieve <account>");
            } else {
                let account = args[2].clone();
                if let Some(entry) = PasswordEntry::retrieve(account) {
                    println!("Account: {}", entry.account);
                    println!("Username: {}", entry.username);
                    println!("Password: {}", entry.password);
                } else {
                    println!("No entry found for the specified account.");
                }
            }
        },
        "list" => {
            let accounts = PasswordEntry::list_accounts();
            if accounts.is_empty() {
                println!("No accounts saved.");
            } else {
                println!("Saved accounts:");
                for account in accounts {
                    println!("- {}", account);
                }
            }
        },
        _ => println!("Unknown command: {}", command),
    }
}
