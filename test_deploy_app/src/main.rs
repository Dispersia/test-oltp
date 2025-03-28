use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use axum_prometheus::PrometheusMetricLayer;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String
}