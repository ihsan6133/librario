//! Global State stored by Librario

use std::{path::PathBuf, sync::Arc};

use tauri::async_runtime::{Mutex, JoinHandle};
use tokio::sync::Semaphore;

use crate::album::AlbumMap;

pub struct State {
    pub wait_handle: Option<Mutex<JoinHandle<AlbumMap>>>,
    pub album_map: Mutex<Option<AlbumMap>>,
    pub app_data_dir: Option<PathBuf>,
    pub thumbnail_semapohore: Arc<Semaphore>,
}

impl State {
    pub fn new() -> State {
        State { wait_handle: None, album_map: Mutex::new(None), app_data_dir: None, thumbnail_semapohore: Arc::new(Semaphore::new(4)) }
    }
}