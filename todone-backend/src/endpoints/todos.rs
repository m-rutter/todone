use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::{types::Uuid, PgPool};

use crate::{
    error::Result,
    jwt::Claims,
    models::todo::{NewTodo, Todo, UpdateTodo},
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/todo/:todo_id", get(get_todo).patch(update_todo))
        .route("/todo", get(get_todos).post(create_todo))
}

async fn get_todos(State(pool): State<PgPool>, claims: Claims) -> Result<Json<Vec<Todo>>> {
    let todos = Todo::get_list(&pool, &claims).await?;

    Ok(Json(todos))
}

async fn get_todo(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = Todo::get(&pool, &claims, todo_id).await?;

    Ok(Json(todo))
}

async fn update_todo(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(todo_id): Path<Uuid>,
    Json(todo_update): Json<UpdateTodo>,
) -> Result<Json<Todo>> {
    Todo::update(&pool, &claims, todo_id, todo_update).await?;

    let todo = Todo::get(&pool, &claims, todo_id).await?;

    Ok(Json(todo))
}

async fn create_todo(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>> {
    let todo = Todo::create(&pool, &claims, new_todo).await?;

    Ok(Json(todo))
}
