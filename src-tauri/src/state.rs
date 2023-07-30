//! Global State stored by Librario

use tauri::async_runtime::{Mutex, JoinHandle};

use crate::album::AlbumMap;

pub struct State {
    pub wait_handle: Option<Mutex<JoinHandle<AlbumMap>>>,
    pub album_map: Mutex<Option<AlbumMap>>
}

impl State {
    pub fn new() -> State {
        State { wait_handle: None, album_map: Mutex::new(None) }
    }
}