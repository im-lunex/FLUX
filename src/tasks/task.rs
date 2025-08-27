use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub completed: bool,
    pub created_at: String,
}

impl Task {
    pub fn new(content: String) -> Self {
        Self {
            id: chrono::Utc::now().timestamp().to_string(),
            content,
            completed: false,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn to_file_line(&self) -> String {
        format!(
            "task_{}: {} | status: {} | created: {}",
            self.id,
            self.content,
            if self.completed { "done" } else { "pending" },
            self.created_at
        )
    }
}
