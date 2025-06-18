pub mod models;

use anyhow::Result;
use chrono::{DateTime, Utc};
use models::{TaskFrequency, TaskStatus, TaskRow, Task, TaskCreate, TaskUpdate};
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

pub struct TaskService {
    pool: PgPool,
}

impl TaskService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_task(&self, task_create: TaskCreate) -> Result<Task> {
        let id: Uuid = Uuid::new_v4();
        let now: DateTime<Utc> = Utc::now();

        let task_row = sqlx::query_as!(
            TaskRow,
            r#"
            INSERT INTO tasks (id, name, description, status, created_at, updated_at, due_date, frequency, recurrence_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            id,
            task_create.name,
            task_create.description,
            TaskStatus::Pending.to_string(),
            now,
            now,
            task_create.due_date,
            task_create.frequency.to_string(),
            task_create.recurrence_date,
        )
            .fetch_one(&self.pool)
            .await?;
        
        todo!()
    }
}