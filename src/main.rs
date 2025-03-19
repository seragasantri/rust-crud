mod handlers;
mod models;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use handlers::{
    create_todo, delete_todo, get_todo, get_todos, update_todo, get_users, get_posts, create_post, get_post, update_post, delete_post // Added get_users here
};
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any}; 

#[tokio::main]
async fn main() {
    // Buat state aplikasi
    let shared_state = Arc::new(Mutex::new(handlers::AppData {
        todos: Vec::new(),
        users: Vec::new(),
        posts: Vec::new(),
    }));
    
    // Konfigurasi CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Buat router
    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(get_todo))
        .route("/todos/{id}", put(update_todo))
        .route("/todos/{id}", delete(delete_todo))
        .route("/users", get(get_users))
        .route("/posts", get(get_posts))
        .route("/posts", post(create_post))
        .route("/posts/{id}", get(get_post))
        .route("/posts/{id}", put(update_post))
        .route("/posts/{id}", delete(delete_post))
        .with_state(shared_state)
        .layer(cors);
    
    // Jalankan server
    let addr = "127.0.0.1:3000";
    println!("Server berjalan di http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}