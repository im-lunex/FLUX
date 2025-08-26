use std::fs;
use std::io::{self, Write};

use inquire::Text;

use crate::auth::{authenticate_user, create_user};
use crate::tasks::statistics::show_task_statistics;
use crate::tasks::storage::{get_all_tasks, save_tasks};
use crate::tasks::task::Task;
use crate::utils::validation::validate_task_content;

// === User Management UI ===
pub fn handle_user_creation() {
    println!("\n--- Create New User ---");

    print!("Enter your username: ");
    try_flush_stdout();
    let mut username = String::new();
    try_read_line(&mut username);

    let username = username.trim().to_string();

    print!("Enter your password: ");
    try_flush_stdout();
    let mut password = String::new();
    try_read_line(&mut password);

    let password = password.trim().to_string();

    match create_user(username, password) {
        Ok(message) => println!("[OK] {}", message),
        Err(e) => eprintln!("[ERR] {}", e),
    }
}

pub fn handle_login() {
    println!("\n--- Login ---");

    print!("Username: ");
    try_flush_stdout();
    let mut username = String::new();
    try_read_line(&mut username);

    let username = username.trim().to_string();

    print!("Password: ");
    try_flush_stdout();
    let mut password = String::new();
    try_read_line(&mut password);
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
        println!("[4] Edit task");
        println!("[5] Mark task as done");
        println!("[6] Mark task as pending");
        println!("[7] Search tasks");
        println!("[8] Task statistics");
        println!("[9] Export to JSON");
        println!("[0] Logout");
        print!("-> Enter choice [1-9]: ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: failed to flush stdout: {}", e);
        }

        let mut command = String::new();
        if let Err(e) = io::stdin().read_line(&mut command) {
            eprintln!("Failed to read line: {}, Please try again!", e);
            continue;
        }

        match command.trim() {
            "1" => view_task(&username),
            "2" => add_task(&username),
            "3" => delete_task(&username),
            "4" => edit_task(&username),
            "5" => mark_task_done(&username),
            "6" => mark_task_pending(&username),
            "7" => search_tasks(&username),
            "8" => show_task_statistics(&username),
            "9" => export_to_json(&username),
            "0" => {
                println!("Logged out.");
                break;
            }
            _ => println!("Invalid choice. Enter 1-9."),
        }
    }
}

// === Task Operations ===
pub fn edit_task(username: &str) {
    println!("\n--- Edit Task ---");
    let mut tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("(no tasks to edit)");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        let status_icon = if task.completed { "[✔]" } else { "[ ]" };
        println!("{}. {} {}", index + 1, status_icon, task.content);
    }

    print!("Enter task number to edit (1-{}): ", tasks.len());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("[ERR] Failed to read input.");
        return;
    }

    let task_num: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= tasks.len() => num,
        _ => {
            println!("[ERR] Invalid task number.");
            return;
        }
    };

    let current_task = &tasks[task_num - 1];

    let edit_content = Text::new("Edit task:")
        .with_initial_value(&current_task.content)
        .prompt();

    match edit_content {
        Ok(new_content) => match validate_task_content(&new_content) {
            Ok((valid_content, warning)) => {
                if let Some(msg) = warning {
                    println!("[WARN] {}", msg);
                }

                tasks[task_num - 1].content = valid_content;
                match save_tasks(username, &tasks) {
                    Ok(()) => println!("[OK] Task updated."),
                    Err(e) => eprintln!("[ERR] Could not save task: {}", e),
                }
            }
            Err(err_msg) => println!("[ERR] {}", err_msg),
        },
        Err(_) => println!("[ERR] Task editing cancelled."),
    }
}

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
    try_flush_stdout();

    let mut task_content = String::new();
    try_read_line(&mut task_content);

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
    try_flush_stdout();
    let mut input = String::new();
    try_read_line(&mut input);

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
    try_flush_stdout();
    let mut input = String::new();
    try_read_line(&mut input);

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
    try_flush_stdout();
    let mut search_term = String::new();
    try_read_line(&mut search_term);
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
    let (json_str, is_success) = match serde_json::to_string_pretty(&tasks) {
        Ok(json_str) => (json_str, true),
        Err(e) => (e.to_string(), false),
    };
    if !is_success {
        eprintln!("Serde Json failed: {}", json_str);
        return;
    }
    let parsed_filename = format!("{}_tasks.json", username.trim());

    match fs::write(parsed_filename.clone(), json_str) {
        Ok(_) => println!("[OK] Tasks exported to '{}'", parsed_filename),
        Err(_) => eprintln!("[ERR] Could not write JSON file."),
    }
}

fn try_flush_stdout() {
    if let Err(e) = io::stdout().flush() {
        eprintln!("Warning: failed to flush stdout: {}", e);
    }
}
fn try_read_line(buf: &mut String) {
    if let Err(e) = io::stdin().read_line(buf) {
        eprintln!("Error: failed to read line: {}", e);
    }
}
