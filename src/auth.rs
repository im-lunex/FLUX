use std::{fs, io::{self, Write}};
use crate::task::task_management_menu;


#[derive(Debug, Clone)]
struct UserAuth {
    username: String,
    password: String,
}

pub fn create_user(username: String, password: String) -> Result<String, std::io::Error> {
    // Validation
    if username.trim().is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Username cannot be empty",
        ));
    }

    if password.trim().is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Password cannot be empty",
        ));
    }

    if username.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Username contains invalid characters",
        ));
    }

    let filename = format!("{}.txt", username.trim());

    // Check if user already exists
    if fs::metadata(&filename).is_ok() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "User already exists",
        ));
    }

    let file_content = format!(
        "Username: {}\nPassword: {}\nCreated: {}\n",
        username.trim(),
        password.trim(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
    );

    fs::write(&filename, file_content)?;

    Ok(format!("User '{}' created successfully!", username.trim()))
}

pub fn authenticate_user(username: &str, password: &str) -> bool {
    let filename = format!("{}.txt", username);

    match fs::read_to_string(&filename) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() >= 2 {
                let stored_username = lines[0].strip_prefix("Username: ").unwrap_or("");
                let stored_password = lines[1].strip_prefix("Password: ").unwrap_or("");
                stored_username == username && stored_password == password
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub fn handle_login() {
    println!("\n=== Login ===");

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    if authenticate_user(&username, &password) {
        println!("✅ Login successful!");
        task_management_menu(username);
    } else {
        println!("❌ Invalid username or password!");
    }
}

pub fn handle_user_creation() {
    println!("\n=== Create New User ===");

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    match create_user(username, password) {
        Ok(message) => println!("✅ {}", message),
        Err(e) => eprintln!("❌ Error creating user: {}", e),
    }
}
