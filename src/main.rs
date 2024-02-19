
use axum::{
    http::StatusCode, routing::{get, post}, BoxError, Json, Router
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), BoxError>{
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(|| async { "Hello, world" }));

    info!("Starting server...");
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}