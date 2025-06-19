use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    handlers::AppState,
    models::{TaskCreate, TaskUpdate, TaskStatus, TaskFrequency},
    calendar::CalendarService,
};

#[derive(Deserialize)]
pub struct TaskFilters {
    status: Option<TaskStatus>,
    frequency: Option<TaskFrequency>,
    name: Option<String>,
    created_start_date: Option<DateTime<Utc>>,
    created_end_date: Option<DateTime<Utc>>,
    updated_start_date: Option<DateTime<Utc>>,
    updated_end_date: Option<DateTime<Utc>>,
    due_start_date: Option<DateTime<Utc>>,
    due_end_date: Option<DateTime<Utc>>,
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(task_create): Json<TaskCreate>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.task_service.create_task(task_create).await {
        Ok(task) => Ok((StatusCode::CREATED, Json(task))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn get_tasks(
    State(state): State<Arc<AppState>>,
    Query(filters): Query<TaskFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.task_service.get_tasks_with_filter(
        filters.status,
        filters.frequency,
        filters.name,
        filters.created_start_date,
        filters.created_end_date,
        filters.updated_start_date,
        filters.updated_end_date,
        filters.due_start_date,
        filters.due_end_date,
    ).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.task_service.get_task_by_id(id).await {
        Ok(Some(task)) => Ok(Json(task)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(task_update): Json<TaskUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.task_service.update_task(id, task_update).await {
        Ok(Some(task)) => Ok(Json(task)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.task_service.delete_task(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_calendar(
    State(state): State<Arc<AppState>>,
    Path((year, month)): Path<(i32, u32)>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get all tasks for the month
    let tasks = match state.task_service.get_tasks_with_filter(
        None, None, None, None, None, None, None, None, None
    ).await {
        Ok(tasks) => tasks,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let calendar = CalendarService::generate_month_view(year, month, tasks);
    Ok(Json(calendar))
}