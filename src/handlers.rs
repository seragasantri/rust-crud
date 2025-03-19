use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::{Arc, Mutex};

use crate::models::{CreatePost, CreateTodo, Post, Todo, UpdatePost, UpdateTodo, Users};

pub struct  AppData {
    pub todos: Vec<Todo>,
    pub users: Vec<Users>,
    pub posts: Vec<Post>
}

pub type AppState = Arc<Mutex<AppData>>;


pub async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let data = state.lock().unwrap();
    Json(data.todos.clone())
}


pub async fn get_todo(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = state.lock().unwrap();
    let todo = todos.todos.iter().find(|todo| todo.id == id);
    
    match todo {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_todo(
    State(state): State<AppState>, 
    Json(payload): Json<CreateTodo>
) -> (StatusCode, Json<Todo>) {
    let mut todos = state.lock().unwrap();
    let id = todos.todos.len() as u64 + 1;
    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };
    
    todos.todos.push(todo.clone());
    
    (StatusCode::CREATED, Json(todo))
}

// Update a todo
pub async fn update_todo(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.lock().unwrap();
    let todo = todos.todos.iter_mut().find(|todo| todo.id == id);
    
    match todo {
        Some(todo) => {
            if let Some(title) = payload.title {
                todo.title = title;
            }
            
            if let Some(completed) = payload.completed {
                todo.completed = completed;
            }
            Ok(Json(todo.clone()))
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Delete a todo
pub async fn delete_todo(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut todos = state.lock().unwrap();
    
    let position = todos.todos.iter().position(|todo| todo.id == id);
    
    match position {
        Some(index) => {
            todos.todos.remove(index);
            StatusCode::NO_CONTENT
        },
        None => StatusCode::NOT_FOUND,
    }
}

// get all users
pub async fn get_users(State(state): State<AppState>) -> Json<Vec<Users>> {
    let data = state.lock().unwrap();
    Json(data.users.clone())
}

// get all post
pub  async  fn get_posts(State(state): State<AppState>) -> Json<Vec<Post>> {
    let data = state.lock().unwrap();
    Json(data.posts.clone())
}

pub async  fn get_post(
    Path(id): Path<u64>,
    State(state): State<AppState>
) -> Result<Json<Post>, StatusCode>{
    let data = state.lock().unwrap();
    let post = data.posts.iter().find(|post| post.id == id);


    match post {
        Some(post) => Ok(Json(post.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async  fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>
) -> (StatusCode, Json<Post>) {
    let mut data = state.lock().unwrap();
    let id = data.posts.len() as u64 + 1;

    let post = Post {
        id,
        title: payload.title,
        body: payload.body,
    };
    data.posts.push(post.clone());

    (StatusCode::CREATED, Json(post))
}

pub  async  fn update_post(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdatePost>
)-> Result<Json<Post>, StatusCode> {
    let mut data = state.lock().unwrap();

    let post = data.posts.iter_mut().find(|post | post.id == id);

    match  post {
        Some(post) => {
            if let Some(title) = payload.title {
                post.title = title;
            }

            if let Some(completed) = payload.body {
                post.body = completed;
            }
            Ok(Json(post.clone()))
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_post (
    Path(id): Path<u64>,
    State(state): State<AppState>
)-> StatusCode {
    let mut data = state.lock().unwrap();
    let posision = data.posts.iter().position(|data| data.id == id);

    match posision {
        Some(index) => {
            data.posts.remove(index);
            StatusCode::NO_CONTENT
        },
        None => StatusCode::NOT_FOUND
    }
}