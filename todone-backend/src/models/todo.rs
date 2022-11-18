use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool, Postgres};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::jwt::Claims;

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
    pub complete: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub content: String,
}

#[derive(Debug, Deserialize)]

pub struct UpdateTodo {
    pub content: Option<String>,
    pub complete: Option<bool>,
}

impl Todo {
    pub async fn get_list(db: &PgPool, claims: &Claims) -> Result<Vec<Todo>> {
        let todos = query_as!(
            Todo,
            r#"
        select 
            todo_id as id, content, complete 
        from "todo" 
        where user_id = $1"#,
            claims.sub
        )
        .fetch_all(db)
        .await?;

        Ok(todos)
    }

    pub async fn get(db: &PgPool, claims: &Claims, todo_id: Uuid) -> Result<Todo> {
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
        .fetch_optional(db)
        .await?
        .ok_or(Error::NotFound)?;

        Ok(todo)
    }

    pub async fn update(
        db: &PgPool,
        claims: &Claims,
        todo_id: Uuid,
        todo_update: UpdateTodo,
    ) -> Result<()> {
        let mut transaction = db.begin().await?;

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
        };

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
        };

        transaction.commit().await?;

        Ok(())
    }

    pub async fn create(
        db: impl sqlx::Executor<'_, Database = Postgres>,
        claims: &Claims,
        new_todo: NewTodo,
    ) -> Result<Todo> {
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
        .fetch_one(db)
        .await?;

        Ok(todo)
    }
}
