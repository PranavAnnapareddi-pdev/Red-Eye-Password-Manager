mod cli;
mod password_manager;
mod utils;


fn main() {
    println!("Hello, world!");
    cli::parse_arguments();
}
