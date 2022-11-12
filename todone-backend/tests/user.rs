mod common;

use sqlx::PgPool;

use todone_backend::endpoints::app;

use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use std::borrow::BorrowMut;

use common::RequestBuilderExt;
use serde_json::json;

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user(db: PgPool) {
    let mut app = app(db).into_service();

    let res = app
        .borrow_mut()
        .oneshot(Request::post("/register").json(json! {{
            "username": "fooBarBaz",
            "password": "password123"
        }}))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}
