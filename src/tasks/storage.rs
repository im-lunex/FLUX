use crate::tasks::task::Task;
use std::fs;

pub fn parse_task_line_advanced(line: &str) -> Option<Task> {
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

pub fn parse_task_line_simple(line: &str) -> Option<Task> {
    let after_prefix = line.strip_prefix("task_")?;
    let (id_part, content_part) = after_prefix.split_once(": ")?;
    Some(Task {
        id: id_part.trim().to_string(),
        content: content_part.trim().to_string(),
        completed: false,
        created_at: "Unknown".to_string(),
    })
}

pub fn parse_task_line(line: &str) -> Option<Task> {
    parse_task_line_advanced(line).or_else(|| parse_task_line_simple(line))
}

pub fn get_all_tasks(username: &str) -> Vec<Task> {
    let filename = format!("{}.txt", username.trim());
    match fs::read_to_string(&filename) {
        Ok(content) => content.lines().filter_map(parse_task_line).collect(),
        Err(_) => Vec::new(),
    }
}

pub fn save_tasks(username: &str, tasks: &[Task]) -> Result<(), std::io::Error> {
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
