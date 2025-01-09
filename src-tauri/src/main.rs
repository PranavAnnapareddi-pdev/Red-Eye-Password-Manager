#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::command;

#[command]
fn add_password(account: String, username: String, password: String) -> String {
  // Replace this logic with your PasswordEntry::add logic
  println!("Adding password for account: {}", account);
  format!("Password for '{}' added successfully!", account)
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![add_password])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
