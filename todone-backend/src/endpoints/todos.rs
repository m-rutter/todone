use axum::{extract::Path, routing::get, Extension, Json, Router};
use sqlx::{types::Uuid, PgPool};

use crate::{
    error::Result,
    jwt::Claims,
    models::todo::{NewTodo, Todo, UpdateTodo},
};

pub fn router() -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo).patch(update_todo))
        .route("/todo", get(get_todos).post(create_todo))
}

async fn get_todos(db: Extension<PgPool>, claims: Claims) -> Result<Json<Vec<Todo>>> {
    let todos = Todo::get_list(&db.0, &claims).await?;

    Ok(Json(todos))
}

async fn get_todo(
    db: Extension<PgPool>,
    claims: Claims,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = Todo::get(&db.0, &claims, todo_id).await?;

    Ok(Json(todo))
}

async fn update_todo(
    db: Extension<PgPool>,
    claims: Claims,
    Path(todo_id): Path<Uuid>,
    Json(todo_update): Json<UpdateTodo>,
) -> Result<Json<Todo>> {
    Todo::update(&db.0, &claims, todo_id, todo_update).await?;

    let todo = Todo::get(&db.0, &claims, todo_id).await?;

    Ok(Json(todo))
}

async fn create_todo(
    db: Extension<PgPool>,
    claims: Claims,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>> {
    let todo = Todo::create(&db.0, &claims, new_todo).await?;

    Ok(Json(todo))
}
