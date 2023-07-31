//! Global State stored by Librario

use std::path::PathBuf;

use tauri::async_runtime::{Mutex, JoinHandle};

use crate::album::AlbumMap;

pub struct State {
    pub wait_handle: Option<Mutex<JoinHandle<AlbumMap>>>,
    pub album_map: Mutex<Option<AlbumMap>>,
    pub app_data_dir: Option<PathBuf>,
}

impl State {
    pub fn new() -> State {
        State { wait_handle: None, album_map: Mutex::new(None), app_data_dir: None }
    }
}