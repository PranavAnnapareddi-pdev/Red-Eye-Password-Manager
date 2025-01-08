use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub account: String,
    pub username: String, // New field for the username
    pub password: String,
}


impl PasswordEntry {
    pub fn add(account: String, username: String, password: String) {
        let new_entry = PasswordEntry {
            account,
            username,
            password,
        };

        // File path where passwords will be stored
        let file_path = "passwords.json";

        // Load existing data from the file
        let mut data = Vec::new();
        if let Ok(mut file) = OpenOptions::new().read(true).write(true).create(true).open(file_path) {
            file.read_to_end(&mut data).unwrap_or_default();
        }

        // Parse existing entries or start with an empty list
        let mut entries: Vec<PasswordEntry> = if data.is_empty() {
            Vec::new()
        } else {
            serde_json::from_slice(&data).unwrap_or_else(|_| Vec::new())
        };

        // Add the new entry
        entries.push(new_entry);

        // Serialize updated entries to JSON
        let json = serde_json::to_string_pretty(&entries).unwrap();

        // Write the updated JSON back to the file
        fs::write(file_path, json).expect("Unable to save password entries.");
    }


    pub fn retrieve(account: String) -> Option<PasswordEntry> {
        let file_path = "passwords.json";
        let mut data = Vec::new();

        // Try reading the file
        if let Ok(mut file) = std::fs::File::open(file_path) {
            file.read_to_end(&mut data).unwrap_or_default();
        } else {
            return None; // Return None if the file doesn't exist
        }

        // Parse the file content into a vector of PasswordEntry
        let entries: Vec<PasswordEntry> = serde_json::from_slice(&data).unwrap_or_else(|_| Vec::new());

        // Search for the account
        for entry in entries {
            if entry.account == account {
                return Some(entry); // Return the matching entry
            }
        }

        None // Return None if no match is found
    }

    pub fn list_accounts() -> Vec<String> {
        let file_path = "passwords.json";
        let mut data = Vec::new();

        // Try reading the file
        if let Ok(mut file) = std::fs::File::open(file_path) {
            file.read_to_end(&mut data).unwrap_or_default();
        } else {
            return Vec::new(); // Return an empty list if the file doesn't exist
        }

        // Parse the file content into a vector of PasswordEntry
        let entries: Vec<PasswordEntry> = serde_json::from_slice(&data).unwrap_or_else(|_| Vec::new());

        // Extract account names
        entries.into_iter().map(|entry| entry.account).collect()
    }





}
