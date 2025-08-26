use flux::*;
use std::fs;

fn cleanup_user_file(username: &str) {
    let filename = format!("{}.txt", username);
    let _ = fs::remove_file(filename);
    let json_file = format!("{}_tasks.json", username);
    let _ = fs::remove_file(json_file);
}

fn setup_user(username: &str, password: &str, initial_tasks: Vec<Task>) {
    cleanup_user_file(username);
    create_user(username.to_string(), password.to_string()).unwrap();
    if !initial_tasks.is_empty() {
        save_tasks(username, &initial_tasks).unwrap();
    }
}

#[test]
fn test_user_creation_and_authentication() {
    let username = "testuser";
    let password = "pass123";
    setup_user(username, password, vec![]);

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
    setup_user(username, password, vec![]);

    let new_task = Task::new("Do homework".into());
    let mut tasks = get_all_tasks(username);
    tasks.push(new_task.clone());
    save_tasks(username, &tasks).unwrap();

    let loaded_tasks = get_all_tasks(username);
    assert_eq!(loaded_tasks.len(), 1);
    assert_eq!(loaded_tasks[0].content, "Do homework");

    cleanup_user_file(username);
}

#[test]
fn test_mark_done_and_pending() {
    let username = "markuser";
    let password = "pass123";
    setup_user(username, password, vec![Task::new("Test task".into())]);

    // Mark done
    let mut tasks = get_all_tasks(username);
    tasks[0].completed = true;
    save_tasks(username, &tasks).unwrap();
    assert!(get_all_tasks(username)[0].completed);

    // Mark pending
    let mut tasks = get_all_tasks(username);
    tasks[0].completed = false;
    save_tasks(username, &tasks).unwrap();
    assert!(!get_all_tasks(username)[0].completed);

    cleanup_user_file(username);
}

#[test]
fn test_export_to_json() {
    let username = "jsonuser";
    let password = "pass123";
    let mut tasks = vec![Task::new("T1".into()), Task::new("T2".into())];
    tasks[0].completed = true;
    setup_user(username, password, tasks);

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

#[test]
fn test_validate_task_normal_length() {
    let input = "Read Rust book";
    let result = validate_task_content(input);
    assert!(result.is_ok());
    let (content, warning) = result.unwrap();
    assert_eq!(content, "Read Rust book");
    assert!(warning.is_none());
}

#[test]
fn test_edit_task() {
    let username = "taskuser_edit";
    let password = "pass123";
    setup_user(username, password, vec![Task::new("Old Content".into())]);

    let mut tasks = get_all_tasks(username);
    assert_eq!(tasks[0].content, "Old Content");

    edit_task_content(&mut tasks, 0, "Updated Content").unwrap();
    save_tasks(username, &tasks).unwrap();

    let loaded_tasks = get_all_tasks(username);
    assert_eq!(loaded_tasks[0].content, "Updated Content");

    cleanup_user_file(username);
}
