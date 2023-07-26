//! Global State stored by Librario

use tauri::async_runtime::Mutex;

use crate::album::AlbumMap;

pub struct State {
    pub albums: Mutex<AlbumMap>,
}
