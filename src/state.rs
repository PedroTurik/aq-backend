use std::{collections::{HashMap, VecDeque}, sync::{atomic::{AtomicUsize, Ordering}, Arc}};
use serde::Serialize;
use tokio::sync::RwLock;


pub static ROOMS_COUNT: AtomicUsize = AtomicUsize::new(0);


#[derive(Serialize, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub score: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct AnimeOpening {
    pub title: String,
    pub artist: String,
    pub video_url: String,
    pub anime_name: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct RoomData {
    pub owner: String,
    pub anime_opening: AnimeOpening,
    pub users: Vec<Player>,
}

#[repr(transparent)]
#[derive(Default, Clone, Debug)]
pub struct RoomStore {
    pub rooms: Arc<RwLock<HashMap<usize, RoomData>>>,
}

impl RoomStore {
    pub async fn insert(&self, room_id: usize, room_data: RoomData) {
        self.rooms.write().await.insert(room_id, room_data);
        ROOMS_COUNT.fetch_add(1, Ordering::Relaxed);
    }

    pub async fn get(&self, room: usize) -> Option<RoomData> {
        self.rooms.read().await.get(&room).map(|v| v.clone())
    }
}