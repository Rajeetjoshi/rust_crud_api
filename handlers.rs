use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{User, CreateUser};

// Get all users
pub async fn get_users(pool: web::Data<PgPool>) -> HttpResponse {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(users)
}

// Create a new user
pub async fn create_user(user: web::Json<CreateUser>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"
    )
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().json(new_user)
}

// You can similarly implement get_user, update_user, delete_user using SQL queries.
