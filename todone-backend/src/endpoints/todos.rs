use axum::{extract::Path, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::Uuid, FromRow, PgPool};

use crate::{error::Result, jwt::Claims};

pub fn router() -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo).patch(update_todo))
        .route("/todo", get(get_todos).post(create_todo))
}

async fn get_todos(db: Extension<PgPool>, claims: Claims) -> Result<Json<Vec<Todo>>> {
    let todos = query_as!(
        Todo,
        r#"
        select 
            todo_id as id, content, complete 
        from "todo" 
        where user_id = $1"#,
        claims.sub
    )
    .fetch_all(&db.0)
    .await?;

    Ok(Json(todos))
}

async fn get_todo(
    db: Extension<PgPool>,
    claims: Claims,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = query_as!(
        Todo,
        r#"
        select 
            todo_id as id, content, complete 
        from "todo" 
        where user_id = $1 and todo_id = $2"#,
        claims.sub,
        todo_id
    )
    .fetch_one(&db.0)
    .await?;

    Ok(Json(todo))
}

async fn update_todo(
    db: Extension<PgPool>,
    claims: Claims,
    Path(todo_id): Path<Uuid>,
    Json(todo_update): Json<UpdateTodo>,
) -> Result<Json<Todo>> {
    let mut transaction = db.0.begin().await?;

    if let Some(content) = todo_update.content {
        query!(
            r#"
        update "todo" 
        set
            content = $1
        where user_id = $2 and todo_id = $3
    "#,
            content,
            claims.sub,
            todo_id
        )
        .execute(&mut transaction)
        .await?;
    }

    if let Some(complete) = todo_update.complete {
        query!(
            r#"
        update "todo" 
        set
            complete = $1
        where user_id = $2 and todo_id = $3
    "#,
            complete,
            claims.sub,
            todo_id
        )
        .execute(&mut transaction)
        .await?;
    }

    let todo = query_as!(
        Todo,
        r#"
        select 
            todo_id as id, content, complete 
        from "todo" 
        where user_id = $1 and todo_id = $2"#,
        claims.sub,
        todo_id
    )
    .fetch_one(&mut transaction)
    .await?;

    Ok(Json(todo))
}

async fn create_todo(
    db: Extension<PgPool>,
    claims: Claims,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>> {
    let todo = query_as!(
        Todo,
        r#"
        insert into "todo" (user_id, content) 
            values ($1, $2)
        returning todo_id as id, content, complete
         "#,
        claims.sub,
        new_todo.content
    )
    .fetch_one(&db.0)
    .await?;

    Ok(Json(todo))
}

#[derive(Debug, Serialize, FromRow)]
struct Todo {
    id: Uuid,
    content: String,
    complete: bool,
}

#[derive(Debug, Deserialize)]
struct NewTodo {
    content: String,
}

#[derive(Debug, Deserialize)]

struct UpdateTodo {
    content: Option<String>,
    complete: Option<bool>,
}
