use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, VecDeque}, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use uuid::Uuid;

const WINDOW_SECONDS: i64 = 300;
const MAX_EVENTS_PER_WINDOW: usize = 5;

#[derive(Clone, Default)]
struct AppState { events: Arc<RwLock<HashMap<String, VecDeque<DateTime<Utc>>>>> }

#[derive(Deserialize)]
struct VerificationRequest { subject_id: String, ip_address: String, country: Option<String> }

#[derive(Serialize)]
struct Decision {
    request_id: Uuid, subject_id: String, decision: String, risk_score: u8,
    reasons: Vec<String>, retry_after_seconds: Option<i64>
}

#[derive(Serialize)]
struct Health { status: &'static str }

async fn health() -> Json<Health> { Json(Health { status: "ok" }) }

async fn verify(State(state): State<AppState>, Json(req): Json<VerificationRequest>)
    -> (StatusCode, Json<Decision>) {
    let now = Utc::now();
    let key = format!("{}:{}", req.subject_id, req.ip_address);
    let mut store = state.events.write().await;
    let events = store.entry(key).or_default();

    while events.front().is_some_and(|t| now.signed_duration_since(*t).num_seconds() >= WINDOW_SECONDS) {
        events.pop_front();
    }

    let mut reasons = Vec::new();
    let mut score = 10u8;
    if req.country.as_deref().is_none() { score += 15; reasons.push("missing_country".into()); }
    if events.len() >= MAX_EVENTS_PER_WINDOW {
        score = 90;
        reasons.push("velocity_limit_exceeded".into());
        let retry = WINDOW_SECONDS - now.signed_duration_since(*events.front().unwrap()).num_seconds();
        return (StatusCode::TOO_MANY_REQUESTS, Json(Decision {
            request_id: Uuid::new_v4(), subject_id: req.subject_id, decision: "review".into(),
            risk_score: score, reasons, retry_after_seconds: Some(retry.max(1))
        }));
    }
    events.push_back(now);
    let decision = if score >= 50 { "review" } else { "allow" };
    (StatusCode::OK, Json(Decision {
        request_id: Uuid::new_v4(), subject_id: req.subject_id, decision: decision.into(),
        risk_score: score, reasons, retry_after_seconds: None
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/verifications", post(verify))
        .layer((TraceLayer::new_for_http(), CorsLayer::permissive()))
        .with_state(AppState::default());
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    info!(%addr, "server started");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
