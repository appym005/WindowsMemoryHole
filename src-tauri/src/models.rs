use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub created_at: i64,
    pub item_type: String,
    pub title: Option<String>,
    pub payload: String,
    pub thumb_path: Option<String>,
    pub pinned: i64,
}
