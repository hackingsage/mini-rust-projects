use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, VecDeque}, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct Job { id: Uuid, task: String, payload: String, status: String, created_at: DateTime<Utc>, attempts: u8 }

#[derive(Clone, Default)]
struct App { jobs: Arc<RwLock<HashMap<Uuid, Job>>>, queue: Arc<Mutex<VecDeque<Uuid>>> }

#[derive(Deserialize)] struct CreateJob { task: String, payload: String }
#[derive(Serialize)] struct Health { status: &'static str }

async fn health() -> Json<Health> { Json(Health { status: "ok" }) }

async fn create(State(app): State<App>, Json(input): Json<CreateJob>) -> (StatusCode, Json<Job>) {
    let job = Job { id: Uuid::new_v4(), task: input.task, payload: input.payload,
        status: "queued".into(), created_at: Utc::now(), attempts: 0 };
    app.jobs.write().await.insert(job.id, job.clone());
    app.queue.lock().await.push_back(job.id);
    (StatusCode::ACCEPTED, Json(job))
}

async fn get_job(State(app): State<App>, Path(id): Path<Uuid>) -> Result<Json<Job>, StatusCode> {
    app.jobs.read().await.get(&id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn worker(app: App) {
    loop {
        let next = app.queue.lock().await.pop_front();
        if let Some(id) = next {
            {
                let mut jobs = app.jobs.write().await;
                if let Some(job) = jobs.get_mut(&id) { job.status = "processing".into(); job.attempts += 1; }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            let mut jobs = app.jobs.write().await;
            if let Some(job) = jobs.get_mut(&id) { job.status = "completed".into(); }
        } else { tokio::time::sleep(tokio::time::Duration::from_millis(250)).await; }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let app = App::default();
    tokio::spawn(worker(app.clone()));
    let api = Router::new().route("/health", get(health))
        .route("/v1/jobs", post(create)).route("/v1/jobs/:id", get(get_job))
        .layer(tower_http::trace::TraceLayer::new_for_http()).with_state(app);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Job queue API: http://localhost:3000");
    axum::serve(listener, api).await.unwrap();
}
