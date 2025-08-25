pub mod user;
use std::fs;

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

pub fn create_user(username: String, password: String) -> Result<String, std::io::Error> {
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
    Ok(format!("User '{}' created.", username.trim()))
}
