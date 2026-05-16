use std::env;
use std::fs;
use std::io::ErrorKind;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    age: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Prevents panic if no command is provided
    if args.len() < 2 {
        println!("No method provided.");
        return;
    }

    // Load existing users from file
    let mut users = match load_users() {
        Ok(users) => users,
        Err(error) => {
            println!("Failed to load users: {}", error);
            return;
        }
    };

    let method = &args[1];

    match method.as_str() {

        // ---------------- ADD USER ----------------
        "add_user" => {
            if args.len() < 5 {
                println!("Usage: add_user <name> <email> <age>");
                return;
            }

            let name = args[2].clone();
            let email = args[3].clone();
            let age_input = &args[4];

            // Parse string into integer safely
            let age: i32 = match age_input.parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid age provided.");
                    return;
                }
            };

            add_user(&mut users, name, email, age);
        }

        // ---------------- VIEW USERS ----------------
        "view_users" => {
            view_users(&users);
        }

        // ---------------- SEARCH USER ----------------
        "search_user" => {
            if args.len() < 3 {
                println!("Usage: search_user <query>");
                return;
            }

            let query = &args[2];

            search_user(&users, query);
        }

        // ---------------- DELETE USER ----------------
        "delete_user" => {
            if args.len() < 3 {
                println!("Usage: delete_user <email>");
                return;
            }

            let email = &args[2];

            delete_user(&mut users, email);
        }

        _ => {
            println!("Invalid method provided.");
        }
    }
}

fn add_user(users: &mut Vec<User>, name: String, email: String, age: i32) {

    // Create new user object
    let user = User {
        name,
        email,
        age,
    };

    // Store inside vector
    users.push(user);

    // Persist to disk
    if let Err(error) = save_users(users) {
        println!("Failed to save users: {}", error);
        return;
    }

    println!("User added successfully.");
}

fn view_users(users: &Vec<User>) {

    // Handle empty state
    if users.is_empty() {
        println!("No users found.");
        return;
    }

    println!("Users:\n");

    // Iterate by borrowing
    for user in users {

        println!(
            "Name: {}, Email: {}, Age: {}",
            user.name,
            user.email,
            user.age
        );
    }
}

fn search_user(users: &Vec<User>, query: &str) {

    let mut found = false;

    for user in users {

        // contains() checks substring match
        if user.name.contains(query) {

            println!(
                "Found User -> Name: {}, Email: {}, Age: {}",
                user.name,
                user.email,
                user.age
            );

            found = true;
        }
    }

    if !found {
        println!("No matching users found.");
    }
}

fn delete_user(users: &mut Vec<User>, email: &str) {

    let original_length = users.len();

    // retain() keeps only items matching condition
    users.retain(|user| user.email != email);

    // Save updated users after deletion
    if let Err(error) = save_users(users) {
        println!("Failed to save users: {}", error);
        return;
    }

    // Compare lengths to know if deletion happened
    if users.len() < original_length {
        println!("User deleted successfully.");
    } else {
        println!("No user found with that email.");
    }
}

fn save_users(users: &Vec<User>) -> Result<(), Box<dyn std::error::Error>> {

    // Convert Rust structs into JSON string
    let json = serde_json::to_string_pretty(users)?;

    // Write JSON string into users.json
    fs::write("users.json", json)?;

    Ok(())
}

fn load_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {

    // Try reading file
    let data = match fs::read_to_string("users.json") {

        // File exists
        Ok(content) => content,

        // File read failed
        Err(error) => {

            // If file does not exist yet
            if error.kind() == ErrorKind::NotFound {

                // Return empty vector instead of crashing
                return Ok(Vec::new());
            }

            // Other errors are real failures
            return Err(Box::new(error));
        }
    };

    // Convert JSON string back into Rust structs
    let users: Vec<User> = serde_json::from_str(&data)?;

    Ok(users)
}