use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}
struct AppState {
    users: Mutex<Vec<User>>,
}

async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

async fn get_user(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == *path) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn create_user(user: web::Json<CreateUser>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let new_user = User {
        id: Uuid::new_v4(),
        name: user.name.clone(),
        email: user.email.clone(),
    };
    users.push(new_user.clone());
    HttpResponse::Created().json(new_user)
}

async fn update_user(path: web::Path<Uuid>, user: web::Json<CreateUser>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if let Some(existing) = users.iter_mut().find(|u| u.id == *path) {
        existing.name = user.name.clone();
        existing.email = user.email.clone();
        return HttpResponse::Ok().json(existing.clone());
    }
    HttpResponse::NotFound().body("User not found")
}

async fn delete_user(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let len_before = users.len();
    users.retain(|u| u.id != *path);
    if users.len() < len_before {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}