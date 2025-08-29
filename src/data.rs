use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub uploaded_at: String, // ISO 8601 格式
}

impl FileEntry {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            uploaded_at: Utc::now().to_rfc3339(),
        }
    }
}