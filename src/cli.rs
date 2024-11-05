use std::env;

pub fn parse_arguments() {
    let args: Vec<String> = env::args().collect();

    // Print out the arguments (for now, just to test)
    if args.len() < 2 {
        println!("Usage: password_manager <command> [arguments]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => println!("Adding a password..."),
        "retrieve" => println!("Retrieving a password..."),
        "list" => println!("Listing passwords..."),
        _ => println!("Unknown command: {}", command),
    }
}
