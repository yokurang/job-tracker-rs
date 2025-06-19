use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskFrequency {
    None,
    Daily, 
    Weekly,
    Monthly,
    Yearly,
    Custom,
    Invalid,
}

impl TaskFrequency {
    pub fn to_string(&self) -> String {
        match self {
            TaskFrequency::None => "none".to_string(),
            TaskFrequency::Daily => "daily".to_string(),
            TaskFrequency::Weekly => "weekly".to_string(),
            TaskFrequency::Monthly => "monthly".to_string(),
            TaskFrequency::Yearly => "yearly".to_string(),
            TaskFrequency::Custom => "custom".to_string(),
            TaskFrequency::Invalid => "invalid".to_string(),
        }
    }
    
    pub fn from_str(s: &str) -> TaskFrequency {
        match s {
            "none" => TaskFrequency::None,
            "daily" => TaskFrequency::Daily,
            "weekly" => TaskFrequency::Weekly,
            "monthly" => TaskFrequency::Monthly,
            "yearly" => TaskFrequency::Yearly,
            "custom" => TaskFrequency::Custom,
            _ => TaskFrequency::Invalid,       
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
    Invalid,
}

impl TaskStatus {
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::InProgress => "in_progress".to_string(),
            TaskStatus::Completed => "completed".to_string(),
            TaskStatus::Cancelled => "cancelled".to_string(),
            TaskStatus::Invalid => "invalid".to_string(),
        }
    }
    
    pub fn from_str(s: &str) -> TaskStatus {
        match s {
            "pending" => TaskStatus::Pending,
            "in_progress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            "cancelled" => TaskStatus::Cancelled,
            _ => TaskStatus::Invalid,
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TaskRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub frequency: String,
    pub recurrence_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub frequency: TaskFrequency,
    pub recurrence_date: Option<DateTime<Utc>>,
}

impl From<TaskRow> for Task {
    fn from(row: TaskRow) -> Task {
        Task {
            id: row.id,
            name: row.name,
            description: row.description,
            status: TaskStatus::from_str(&row.status),
            created_at: row.created_at,
            updated_at: row.updated_at,
            due_date: row.due_date,
            frequency: TaskFrequency::from_str(&row.frequency),
            recurrence_date: row.recurrence_date,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreate {
    pub name: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub frequency: TaskFrequency,
    pub recurrence_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub due_date: Option<DateTime<Utc>>,
    pub frequency: Option<TaskFrequency>,
    pub recurrence_date: Option<DateTime<Utc>>,   
}