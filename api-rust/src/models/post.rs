use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub status: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}