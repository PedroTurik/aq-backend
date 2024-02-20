mod state;

use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use axum::{extract::{Query, State}, routing::get, serve, BoxError, Router};
use serde::Deserialize;
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};

use state::{RoomData, ROOMS_COUNT};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, instrument};

use crate::state::RoomStore;



#[instrument]
async fn on_connect(socket: SocketRef) {
    info!("New connection: {:?}", socket);
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let room_store = RoomStore::default();
    let (layer, io) = SocketIo::builder().with_state(room_store.clone()).build_layer();

    io.ns("/", on_connect);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/load_data", get(get_rooms_count))
        .route("/room_data", get(get_room_data))
        .with_state(room_store.clone())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server...");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}


async fn get_rooms_count() -> String {
    ROOMS_COUNT.load(Ordering::Relaxed).to_string()
}

#[derive(Deserialize)]
struct RoomDataQuery {
    room_id: usize,
}

async fn get_room_data(Query(params): Query<RoomDataQuery>, State(state): State<RoomStore>) -> Option<RoomData> {
    let room = state.get(params.room_id).await;
    room

}