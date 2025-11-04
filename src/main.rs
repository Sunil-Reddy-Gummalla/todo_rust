use rocket::{http::Status, *};
mod task;
use task::*;

use rocket::serde::json::Json;



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello_user, fetch_tasks, create_task,update_task, delete_task])
}

#[get("/home/<name>")]
fn hello_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/tasks")]
fn fetch_tasks() -> Json<Vec<Task>> {
    let tasks = load_tasks();
    Json(tasks)
}

#[post("/create-task", format = "application/json", data = "<task>")]
fn create_task(task: Json<Task>) -> Status {
    let mut tasks = load_tasks();
    if let Some(_t) = tasks.iter().find(|t| t.task_name == task.0.task_name) {
        return Status::Conflict;
    }
    tasks.push(task.0);
    save_tasks(&tasks);
    Status::Created
}

#[put("/update-task", format = "application/json", data = "<task>")]
fn update_task(task: Json<Task>) -> Status {
    let mut tasks = load_tasks();
    if let Some(t) = tasks.iter_mut().position(|t| t.task_name == task.0.task_name) {
        tasks.remove(t);
        tasks.insert(t, task.0);
    } else {
        return Status::NotFound;
    }
    save_tasks(&tasks);
    Status::Accepted
}

#[delete("/delete-task/<task_name>")]
fn delete_task(task_name: &str) -> Status {
    let mut tasks = load_tasks();
    if let Some(t) = tasks.iter_mut().position(|t| t.task_name == task_name) {
        tasks.remove(t);
    } else {
        return Status::NotFound;
    }
    save_tasks(&tasks);
    Status::Accepted
}