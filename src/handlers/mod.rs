pub mod task_handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};

use sqlx::PgPool;
use std::sync::Arc;

use crate::services::TaskService;

pub struct AppState {
    pub task_service: TaskService,
}

pub fn create_router(pool: PgPool) -> Router {
    let task_service = TaskService::new(pool);
    let state = Arc::new(AppState { task_service });

    Router::new()
        .route("/api/tasks", post(task_handlers::create_task))
        .route("/api/tasks", get(task_handlers::get_tasks))
        .route("/api/tasks/:id", get(task_handlers::get_task))
        .route("/api/tasks/:id", put(task_handlers::update_task))
        .route("/api/tasks/:id", delete(task_handlers::delete_task))
        .route("/api/calendar/:year/:month", get(task_handlers::get_calendar))
        .with_state(state)
}