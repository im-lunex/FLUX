use std::io::{self, Write};

#[cfg(test)] 
use todo_cli::auth::{ create_user };
use todo_cli::auth:: { handle_user_creation, handle_login };
#[cfg(test)] 
use todo_cli::task::{ get_all_tasks, save_tasks, Task, parse_task_line_advanced, parse_task_line_simple, export_to_json};

fn main() {
    println!("=== Welcome to Task Manager ===");

    loop {
        println!("\nChoose an option:");
        println!("1. Create new user");
        println!("2. Login");
        println!("3. Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => handle_user_creation(),
            "2" => handle_login(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please enter 1, 2, or 3."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_parsing() {
        // Test new format
        let new_format = "task_123: Buy milk | status: done | created: 2023-01-01 10:00:00";
        let task = parse_task_line_advanced(new_format).unwrap();
        assert_eq!(task.id, "123");
        assert_eq!(task.content, "Buy milk");
        assert!(task.completed);

        // Test old format
        let old_format = "task_456: Call mom";
        let task = parse_task_line_simple(old_format).unwrap();
        assert_eq!(task.id, "456");
        assert_eq!(task.content, "Call mom");
        assert!(!task.completed);
    }

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test task".to_string());
        assert_eq!(task.content, "Test task");
        assert!(!task.completed);
        assert!(!task.id.is_empty());
    }

    #[test]
    fn test_task_export() {
        let test_user = "test_user";
        create_user(test_user.to_string(), String::from("123456")).unwrap();
        let new_task = Task::new("Test task".to_string());
        let mut tasks = get_all_tasks(test_user);
        tasks.push(new_task);
        save_tasks(test_user, &tasks).unwrap();
        export_to_json(test_user);
        let content = std::fs::read_to_string("tasks.json").unwrap();
        let data: Vec<Task> = serde_json::from_str(&content).unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].content, "Test task");
    }
}

