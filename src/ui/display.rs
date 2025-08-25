use std::fs;
use std::io::{self, Write};

use crate::auth::{authenticate_user, create_user};
use crate::tasks::statistics::show_task_statistics;
use crate::tasks::storage::{get_all_tasks, save_tasks};
use crate::tasks::task::Task;
use crate::utils::validation::validate_task_content;

// === User Management UI ===
pub fn handle_user_creation() {
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

pub fn handle_login() {
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

// === Task Management Menu ===
pub fn task_management_menu(username: String) {
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

// === Task Operations ===
pub fn view_task(username: &str) {
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

pub fn add_task(username: &str) {
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

pub fn delete_task(username: &str) {
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

pub fn mark_task_done(username: &str) {
    mark_task_status(username, true, "done");
}

pub fn mark_task_pending(username: &str) {
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

pub fn search_tasks(username: &str) {
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

pub fn export_to_json(username: &str) {
    let tasks = get_all_tasks(username);
    let json_str = serde_json::to_string_pretty(&tasks).unwrap();
    let parsed_filename = format!("{}_tasks.json", username.trim());

    match fs::write(parsed_filename.clone(), json_str) {
        Ok(_) => println!("[OK] Tasks exported to '{}'", parsed_filename),
        Err(_) => eprintln!("[ERR] Could not write JSON file."),
    }
}
