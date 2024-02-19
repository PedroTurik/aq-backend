
use axum::{
    routing::get, BoxError, Router
};
use socketioxide::{extract::SocketRef, socket::DisconnectReason, SocketIo};
use tokio::runtime::EnterGuard;
use tower_http::cors::CorsLayer;
use tracing::{info, instrument, Instrument};

#[instrument]
async fn on_connect(socket: SocketRef) {
    info!("New connection: {:?}", socket);
}


#[tokio::main]
async fn main() -> Result<(), BoxError>{
    // initialize tracing
    tracing_subscriber::fmt::init();

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);



    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(|| async { "Hello, world" }))
        .layer(CorsLayer::permissive())
        .layer(layer);

    info!("Starting server...");
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}