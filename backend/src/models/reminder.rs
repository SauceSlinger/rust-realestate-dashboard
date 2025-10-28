use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reminder {
    pub id: Option<i64>,
    pub property_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub due_date: String,
    pub completed: bool,
    pub reminder_type: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReminder {
    pub property_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub due_date: String,
    pub reminder_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReminder {
    pub property_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub completed: Option<bool>,
    pub reminder_type: Option<String>,
}
