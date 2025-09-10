use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub uploaded_at: String, // ISO 8601 格式
}

impl FileEntry {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            uploaded_at: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
