pub fn validate_task_content(input: &str) -> Result<(String, Option<String>), String> {
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
