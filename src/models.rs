use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

//User

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct  Users {
    pub id: u64,
    pub name: String,
    pub username: String,
    pub password: String
}

// pub struct  CreateUser {
//     pub name: String,
//     pub username: String,
//     pub password: String
// }


//POST
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct  Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub  struct  CreatePost {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub  struct  UpdatePost {
    pub title: Option<String>,
    pub  body: Option<String>
}