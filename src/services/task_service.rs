use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::models::{TaskFrequency, TaskStatus, TaskRow, Task, TaskCreate, TaskUpdate};
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;
use dotenvy::dotenv;

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

        let task_row: TaskRow = sqlx::query_as!(
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
        Ok(Task::from(task_row))
    }

    pub async fn get_task_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        let task_row: Option<TaskRow> = sqlx::query_as!(
            TaskRow,
            r#"
            SELECT * FROM tasks where id = $1
            "#,
            id,
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(task_row.map(Task::from))
    }

    pub async fn get_tasks_with_filter(
        &self,
        status: Option<TaskStatus>,
        frequency: Option<TaskFrequency>,
        name: Option<String>,
        created_start_date: Option<DateTime<Utc>>,
        created_end_date: Option<DateTime<Utc>>,
        updated_start_date: Option<DateTime<Utc>>,
        updated_end_date: Option<DateTime<Utc>>,
        due_start_date: Option<DateTime<Utc>>,
        due_end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Task>> {
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM tasks WHERE 1 = 1");

        if let Some(status) = status {
            builder.push(" AND status = ").push_bind(status.to_string());
        }

        if let Some(frequency) = frequency {
            builder.push(" AND frequency = ").push_bind(frequency.to_string());
        }

        if let Some(name) = name {
            builder.push(" AND name ILIKE ").push_bind(format!("%{}%", name));
        }

        if let Some(created_start_date) = created_start_date {
            builder.push(" AND created_at >= ").push_bind(created_start_date);
        }

        if let Some(created_end_date) = created_end_date {
            builder.push(" AND created_at <= ").push_bind(created_end_date);
        }

        if let Some(updated_start_date) = updated_start_date {
            builder.push(" AND updated_at >= ").push_bind(updated_start_date);
        }

        if let Some(updated_end_date) = updated_end_date {
            builder.push(" AND updated_at <= ").push_bind(updated_end_date);
        }

        if let Some(due_start_date) = due_start_date {
            builder.push(" AND due_date >= ").push_bind(due_start_date);
        }

        if let Some(due_end_date) = due_end_date {
            builder.push(" AND due_date <= ").push_bind(due_end_date);
        }

        let rows = builder
            .build_query_as::<TaskRow>()
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Task::from).collect())
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM tasks where id = $1",
            id
        )
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update_task(&self, id: Uuid, task_update: TaskUpdate) -> Result<Option<Task>> {
        if self.get_task_by_id(id).await?.is_none() {
            return Ok(None);
        }

        let mut query = QueryBuilder::new("UPDATE tasks SET updated_at = NOW()");
        let mut has_updates = false;

        if let Some(name) = task_update.name {
            query.push(", name = ").push_bind(name); has_updates = true;
        }

        if let Some(description) = task_update.description {
            query.push(", description = ").push_bind(description); has_updates = true;
        }

        if let Some(status) = task_update.status {
            query.push(", status = ").push_bind(status.to_string()); has_updates = true;
        }

        if let Some(due_date) = task_update.due_date {
            query.push(", due_date = ").push_bind(due_date); has_updates = true;
        }

        if let Some(frequency) = task_update.frequency {
            query.push(", frequency = ").push_bind(frequency.to_string()); has_updates = true;
        }

        if let Some(recurrence_date) = task_update.recurrence_date {
            query.push(", recurrence_date = ").push_bind(recurrence_date); has_updates = true;
        }

        if !has_updates {
            return self.get_task_by_id(id).await;
        }

        query
            .push("WHERE id = ")
            .push_bind(id)
            .push(" RETURNING *");

        let task_row: Option<TaskRow> = query
            .build_query_as::<TaskRow>()
            .fetch_optional(&self.pool)
            .await?;

        Ok(task_row.map(Task::from))
    }
}