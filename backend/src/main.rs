use std::{fs::{read_to_string, OpenOptions}, arch::x86_64};

use axum::{
    extract::Json,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let _ = OpenOptions::new().create(true).open("todos.json");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/todos", get(get_all_todos))
        .route("/todos", post(post_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(put_todo))
        .route("/todos/:id", patch(patch_todo))
        .route("/todos/:id", delete(delete_todo));

    // `POST /users` goes to `create_user`

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    id: u64,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewToDo {
    title: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditToDo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    count: u64,
}

pub fn read_todos() -> anyhow::Result<Vec<ToDo>> {
    let file_content = read_to_string("todos.json")?;
    let list = serde_json::from_str::<Vec<ToDo>>(&file_content)?;
    Ok(list)
}

pub fn write_todos(list: Vec<ToDo>) -> anyhow::Result<()> {
    let file_content = serde_json::to_string(&list)?;
    std::fs::write("todos.json", file_content)?;
    Ok(())
}

async fn get_all_todos() -> impl IntoResponse {
    let array = read_todos().unwrap();
    Json(array)
}

async fn get_todo(Path(id): Path<u64>) -> impl IntoResponse {
    let array = read_todos().unwrap();
    let todo = array.iter().find(|x| x.id == id);
    match todo {
        Some(x) => Json(x).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn post_todo(Json(payload): Json<NewToDo>) -> impl IntoResponse {
    let mut array: Vec<ToDo> = read_todos().unwrap();
    let info_txt = read_to_string("info.json").unwrap();
    let mut info = serde_json::from_str::<Info>(&info_txt).unwrap();
    let new_id = info.count;
    let new_new_todo = ToDo {
        id: new_id,
        title: payload.title,
        description: payload.description,
        completed: payload.completed,
    };
    info.count = info.count + 1;
    let info_txt = serde_json::to_string(&info).unwrap();
    std::fs::write("info.json", &info_txt);
    array.push(new_new_todo);
    write_todos(array).expect("Post non inviato");
}

async fn patch_todo(Path(id): Path<u64>, Json(payload): Json<EditToDo>) -> impl IntoResponse {

}

async fn put_todo(Path(id): Path<u64>, Json(payload): Json<NewToDo>) -> impl IntoResponse {
    let mut array: Vec<ToDo> = read_todos().unwrap();
    let new_new_todo = ToDo{
        id : id,
        title : payload.title,
        description : payload.description,
        completed : payload.completed,
    };
    let pos = array.iter().position(|x| x.id == id).unwrap();
    array[pos] = new_new_todo;
    write_todos(array).expect("Post non inviato");
}

async fn delete_todo(Path(id): Path<u64>) -> impl IntoResponse {
    let array = read_todos().unwrap(); // Leggi array dei todos
    let array = array.into_iter().filter(|x| x.id != id).collect::<Vec<_>>();
    write_todos(array); // Riscrivi array
}
