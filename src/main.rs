use std::sync::Arc;
use std::{sync::Mutex, vec};
use actix_web::{delete, get, web, post, App, HttpResponse, HttpServer, Responder, patch};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// state
struct AppState {
    tasks: Arc<Mutex<Vec<Task>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new( AppState { tasks: Arc::new(Mutex::new(vec![])) });

    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
        .service(get_tasks)
        .service(create_task)
        .service(toggle_task_status)
        .service(delete_task)
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/tasks")]
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    let body = serde_json::to_string(&tasks.clone()).unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Deserialize)]
struct CreateTaskForm {
    name: String
}

#[post("/tasks")]
async fn create_task(data: web::Data<AppState>, json: web::Json<CreateTaskForm>) -> impl Responder {
    let new_task = Task::new(
      Uuid::new_v4().to_string(),
    json.name.clone(),
    );
    let mut tasks = data.tasks.lock().unwrap();
    let body = serde_json::to_string(&new_task).unwrap();
    tasks.push(new_task);
    HttpResponse::Ok().body(body)
}

#[derive(Deserialize)]
struct TaskIdParam {
    task_id: String
}

#[patch("/tasks/{task_id}")]
async fn toggle_task_status(path: web::Path<TaskIdParam>, data: web::Data<AppState>) -> impl Responder {
    let tasks = &mut data.tasks.lock().unwrap();
    let maybe_index = tasks.iter().position(|v| v.id == path.task_id);

    match maybe_index {
        Some(index) => {
            let target = &mut tasks[index];
            target.toggle_status();
            HttpResponse::Ok().body(serde_json::to_string(target).unwrap())
        },
        None => HttpResponse::NotFound().body(format!("{} is not found", path.task_id)),
    }
}

#[delete("/tasks/{task_id}")]
async fn delete_task(path: web::Path<TaskIdParam>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let maybe_index = tasks.iter().position(|v| v.id == path.task_id);

    match maybe_index {
        Some(index) => {
            tasks.remove(index);
            HttpResponse::Ok().body("")
        },
        None => HttpResponse::NotFound().body(format!("{} is not found", path.task_id)),
    }
}

// model
#[derive(Deserialize, Serialize, Clone)]
struct Task {
    id: String,
    name: String,
    is_finished: bool
    // created_at: DateTime<Utc>
}

impl Task {
    pub fn new(id: String, name: String) -> Task {
        Task { id, name, is_finished: false }
    }
    
    pub fn toggle_status(&mut self) -> () {
        self.is_finished = !self.is_finished;
    }
}