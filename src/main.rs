use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct UserAuth {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: String,
    content: String,
    completed: bool,
    created_at: String,
}

impl Task {
    fn new(content: String) -> Self {
        Self {
            id: chrono::Utc::now().timestamp().to_string(),
            content,
            completed: false,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    fn to_file_line(&self) -> String {
        format!(
            "task_{}: {} | status: {} | created: {}",
            self.id,
            self.content,
            if self.completed { "done" } else { "pending" },
            self.created_at
        )
    }
}

fn main() {
    println!("\n=== FLUX ===");

    loop {
        println!("\nMain Menu:");
        println!("[1] Create new user");
        println!("[2] Login");
        println!("[3] Exit");
        print!("-> Enter your choice [1-3]: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => handle_user_creation(),
            "2" => handle_login(),
            "3" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}

fn handle_user_creation() {
    println!("\n--- Create New User ---");

    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    match create_user(username, password) {
        Ok(message) => println!("[OK] {}", message),
        Err(e) => eprintln!("[ERR] {}", e),
    }
}

fn handle_login() {
    println!("\n--- Login ---");

    print!("Username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    if authenticate_user(&username, &password) {
        println!("[OK] Login successful. Welcome, {}!", username);
        task_management_menu(username);
    } else {
        println!("[ERR] Invalid username or password.");
    }
}

fn authenticate_user(username: &str, password: &str) -> bool {
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

fn task_management_menu(username: String) {
    loop {
        println!("\n--- Task Menu ({}) ---", username);
        println!("[1] View tasks");
        println!("[2] Add task");
        println!("[3] Delete task");
        println!("[4] Mark task as done");
        println!("[5] Mark task as pending");
        println!("[6] Search tasks");
        println!("[7] Task statistics");
        println!("[8] Export to JSON");
        println!("[9] Logout");
        print!("-> Enter choice [1-9]: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "1" => view_task(&username),
            "2" => add_task(&username),
            "3" => delete_task(&username),
            "4" => mark_task_done(&username),
            "5" => mark_task_pending(&username),
            "6" => search_tasks(&username),
            "7" => show_task_statistics(&username),
            "8" => export_to_json(&username),
            "9" => {
                println!("Logged out.");
                break;
            }
            _ => println!("Invalid choice. Enter 1-9."),
        }
    }
}

// === Task Parsing ===
fn parse_task_line_advanced(line: &str) -> Option<Task> {
    let after_prefix = line.strip_prefix("task_")?;
    let parts: Vec<&str> = after_prefix.split(" | ").collect();
    if parts.len() < 3 {
        return None;
    }
    let (id_part, content_part) = parts[0].split_once(": ")?;
    let status_part = parts[1].strip_prefix("status: ")?;
    let completed = status_part == "done";
    let created_part = parts[2].strip_prefix("created: ").unwrap_or("Unknown");

    Some(Task {
        id: id_part.trim().to_string(),
        content: content_part.trim().to_string(),
        completed,
        created_at: created_part.to_string(),
    })
}

fn parse_task_line_simple(line: &str) -> Option<Task> {
    let after_prefix = line.strip_prefix("task_")?;
    let (id_part, content_part) = after_prefix.split_once(": ")?;
    Some(Task {
        id: id_part.trim().to_string(),
        content: content_part.trim().to_string(),
        completed: false,
        created_at: "Unknown".to_string(),
    })
}

fn parse_task_line(line: &str) -> Option<Task> {
    parse_task_line_advanced(line).or_else(|| parse_task_line_simple(line))
}

// === File Handling ===
fn get_all_tasks(username: &str) -> Vec<Task> {
    let filename = format!("{}.txt", username.trim());
    match fs::read_to_string(&filename) {
        Ok(content) => content.lines().filter_map(parse_task_line).collect(),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(username: &str, tasks: &[Task]) -> Result<(), std::io::Error> {
    let filename = format!("{}.txt", username.trim());
    let mut lines = Vec::new();

    if let Ok(content) = fs::read_to_string(&filename) {
        for line in content.lines() {
            if !line.starts_with("task_") {
                lines.push(line.to_string());
            }
        }
    }
    for task in tasks {
        lines.push(task.to_file_line());
    }

    fs::write(&filename, lines.join("\n"))
}

// === Task Operations ===
fn view_task(username: &str) {
    println!("\n--- Your Tasks ---");
    let tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("(no tasks found)");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        let status_icon = if task.completed { "[✔]" } else { "[ ]" };
        let status_text = if task.completed { "DONE" } else { "PENDING" };
        println!(
            "{}. {} {} - {} (id: {}, created: {})",
            index + 1,
            status_icon,
            status_text,
            task.content,
            task.id,
            task.created_at
        );
    }
    println!("Total tasks: {}", tasks.len());
}

fn validate_task_content(input: &str) -> Result<(String, Option<String>), String> {
    let mut trimmed = input.trim().to_string();

    if trimmed.is_empty() {
        return Err("Task description cannot be empty or whitespace.".into());
    }

    if trimmed.len() > 200 {
        trimmed.truncate(200);
        return Ok((
            trimmed,
            Some("Task description too long. Truncated to 200 characters.".into()),
        ));
    }

    Ok((trimmed, None))
}

fn add_task(username: &str) {
    println!("\n--- Add Task ---");
    print!("Task description: ");
    io::stdout().flush().unwrap();

    let mut task_content = String::new();
    io::stdin()
        .read_line(&mut task_content)
        .expect("Failed to read line");

    match validate_task_content(&task_content) {
        Ok((valid_content, warning)) => {
            if let Some(msg) = warning {
                println!("[WARN] {}", msg);
            }

            let new_task = Task::new(valid_content);
            let mut tasks = get_all_tasks(username);
            tasks.push(new_task);

            match save_tasks(username, &tasks) {
                Ok(()) => println!("[OK] Task added."),
                Err(e) => eprintln!("[ERR] Could not save task: {}", e),
            }
        }
        Err(err_msg) => {
            println!("[ERR] {}", err_msg);
        }
    }
}

fn delete_task(username: &str) {
    println!("\n--- Delete Task ---");
    let mut tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("(no tasks to delete)");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        let status_icon = if task.completed { "[✔]" } else { "[ ]" };
        println!("{}. {} {}", index + 1, status_icon, task.content);
    }

    print!("Enter task number to delete (1-{}): ", tasks.len());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(task_num) if task_num > 0 && task_num <= tasks.len() => {
            let removed_task = tasks.remove(task_num - 1);
            match save_tasks(username, &tasks) {
                Ok(()) => println!("[OK] Task '{}' deleted.", removed_task.content),
                Err(e) => eprintln!("[ERR] Could not delete task: {}", e),
            }
        }
        _ => println!("[ERR] Invalid task number."),
    }
}

fn mark_task_done(username: &str) {
    mark_task_status(username, true, "done");
}

fn mark_task_pending(username: &str) {
    mark_task_status(username, false, "pending");
}

fn mark_task_status(username: &str, completed: bool, status_name: &str) {
    println!("\n--- Mark Task as {} ---", status_name.to_uppercase());
    let mut tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("(no tasks available)");
        return;
    }

    let relevant_tasks: Vec<(usize, &Task)> = tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| task.completed != completed)
        .collect();

    if relevant_tasks.is_empty() {
        println!("(no tasks to mark as {})", status_name);
        return;
    }

    for (display_index, (_, task)) in relevant_tasks.iter().enumerate() {
        let status_icon = if task.completed { "[✔]" } else { "[ ]" };
        println!("{}. {} {}", display_index + 1, status_icon, task.content);
    }

    print!("Enter task number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(choice) if choice > 0 && choice <= relevant_tasks.len() => {
            let (original_index, _) = relevant_tasks[choice - 1];
            tasks[original_index].completed = completed;
            match save_tasks(username, &tasks) {
                Ok(()) => println!("[OK] Task marked as {}.", status_name),
                Err(e) => eprintln!("[ERR] Could not update task: {}", e),
            }
        }
        _ => println!("[ERR] Invalid choice."),
    }
}

fn search_tasks(username: &str) {
    println!("\n--- Search Tasks ---");
    print!("Enter search term: ");
    io::stdout().flush().unwrap();
    let mut search_term = String::new();
    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");
    let search_term = search_term.trim().to_lowercase();

    if search_term.is_empty() {
        println!("[ERR] Search term cannot be empty.");
        return;
    }

    let tasks = get_all_tasks(username);
    let matching_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|task| task.content.to_lowercase().contains(&search_term))
        .collect();

    if matching_tasks.is_empty() {
        println!("No tasks found containing '{}'", search_term);
    } else {
        println!("Found {} matching task(s):", matching_tasks.len());
        for (index, task) in matching_tasks.iter().enumerate() {
            let status_icon = if task.completed { "[✔]" } else { "[ ]" };
            println!("{}. {} {}", index + 1, status_icon, task.content);
        }
    }
}

fn show_task_statistics(username: &str) {
    println!("\n--- Task Statistics ---");
    let tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("(no tasks available)");
        return;
    }

    let total = tasks.len();
    let completed = tasks.iter().filter(|task| task.completed).count();
    let pending = total - completed;
    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("Total tasks: {}", total);
    println!("Completed: {}", completed);
    println!("Pending: {}", pending);
    println!("Completion rate: {:.1}%", completion_rate);
}

fn create_user(username: String, password: String) -> Result<String, std::io::Error> {
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

fn export_to_json(username: &str) {
    let tasks = get_all_tasks(username);
    let json_str = serde_json::to_string_pretty(&tasks).unwrap();
    let parsed_filename = format!("{}_tasks.json", username.trim());

    match fs::write(parsed_filename.clone(), json_str) {
        Ok(_) => println!("[OK] Tasks exported to '{}'", parsed_filename),
        Err(_) => eprintln!("[ERR] Could not write JSON file."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn cleanup_user_file(username: &str) {
        let filename = format!("{}.txt", username);
        let _ = fs::remove_file(filename);
        let json_file = format!("{}_tasks.json", username);
        let _ = fs::remove_file(json_file);
    }

    #[test]
    fn test_user_creation_and_authentication() {
        let username = "testuser";
        let password = "pass123";
        cleanup_user_file(username);

        // create user
        let res = create_user(username.to_string(), password.to_string());
        assert!(res.is_ok(), "User creation failed: {:?}", res);

        // authenticate
        assert!(authenticate_user(username, password), "Auth should pass");
        assert!(
            !authenticate_user(username, "wrong"),
            "Wrong password should fail"
        );

        cleanup_user_file(username);
    }

    #[test]
    fn test_add_and_view_task() {
        let username = "taskuser";
        let password = "pass123";
        cleanup_user_file(username);
        create_user(username.to_string(), password.to_string()).unwrap();

        // add task
        let new_task = Task::new("Do homework".into());
        let mut tasks = get_all_tasks(username);
        tasks.push(new_task.clone());
        save_tasks(username, &tasks).unwrap();

        // verify task saved
        let loaded_tasks = get_all_tasks(username);
        assert_eq!(loaded_tasks.len(), 1);
        assert_eq!(loaded_tasks[0].content, "Do homework");

        cleanup_user_file(username);
    }

    #[test]
    fn test_mark_done_and_pending() {
        let username = "markuser";
        let password = "pass123";
        cleanup_user_file(username);
        create_user(username.to_string(), password.to_string()).unwrap();

        // add task
        let mut tasks = vec![Task::new("Test task".into())];
        save_tasks(username, &tasks).unwrap();

        // mark done
        tasks[0].completed = true;
        save_tasks(username, &tasks).unwrap();
        let loaded = get_all_tasks(username);
        assert!(loaded[0].completed);

        // mark pending
        let mut updated = loaded;
        updated[0].completed = false;
        save_tasks(username, &updated).unwrap();
        let reloaded = get_all_tasks(username);
        assert!(!reloaded[0].completed);

        cleanup_user_file(username);
    }

    #[test]
    fn test_export_to_json() {
        let username = "jsonuser";
        let password = "pass123";
        cleanup_user_file(username);
        create_user(username.to_string(), password.to_string()).unwrap();

        // add tasks
        let mut tasks = vec![Task::new("T1".into()), Task::new("T2".into())];
        tasks[0].completed = true;
        save_tasks(username, &tasks).unwrap();

        // export
        export_to_json(username);
        let filename = format!("{}_tasks.json", username);
        let json_content = fs::read_to_string(filename).unwrap();
        assert!(json_content.contains("T1"));
        assert!(json_content.contains("T2"));

        cleanup_user_file(username);
    }

    #[test]
    fn test_validate_task_empty() {
        let result = validate_task_content("");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Task description cannot be empty or whitespace."
        );
    }

    #[test]
    fn test_validate_task_whitespace() {
        let result = validate_task_content("    ");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Task description cannot be empty or whitespace."
        );
    }

    #[test]
    fn test_validate_task_trimmed() {
        let result = validate_task_content("   Buy milk   ");
        assert!(result.is_ok());
        let (content, warning) = result.unwrap();
        assert_eq!(content, "Buy milk");
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_task_too_long() {
        let long_input = "x".repeat(250);
        let result = validate_task_content(&long_input);
        assert!(result.is_ok());
        let (content, warning) = result.unwrap();
        assert_eq!(content.len(), 200);
        assert!(warning.is_some());
        assert_eq!(
            warning.unwrap(),
            "Task description too long. Truncated to 200 characters."
        );
    }
}
