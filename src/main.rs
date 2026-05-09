use std::env;


#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No method provided.");
        return;
    }

    let mut users: Vec<User> = Vec::new();

    let method = &args[1];

    match method.as_str() {
        "add_user" => {
            if args.len() < 5 {
                println!("Usage: add_user <name> <email> <age>");
                return;
            }

            let name = &args[2];
            let email = &args[3];
            let age_input = &args[4];

            let age: i32 = match age_input.parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid age provided.");
                    return;
                }
            };

            add_user(&mut users, name.to_string(), email.to_string(), age);

            view_users(&users);
        }

        "view_users" => {
            view_users(&users);
        }

        "search_user" => {
            if args.len() < 3 {
                println!("Usage: search_user <query>");
                return;
            }

            let query = &args[2];

            search_user(&users, query);
        }

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
    let user = User {
        name,
        email,
        age,
    };

    users.push(user);

    println!("User added successfully.");
}

fn view_users(users: &Vec<User>) {
    if users.is_empty() {
        println!("No users found.");
        return;
    }

    println!("Users:\n");

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

    users.retain(|user| user.email != email);

    if users.len() < original_length {
        println!("User deleted successfully.");
    } else {
        println!("No user found with that email.");
    }
}