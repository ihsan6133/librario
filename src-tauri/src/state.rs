//! Global State stored by Librario

use tauri::async_runtime::{Mutex, JoinHandle};

use crate::album::AlbumMap;

pub struct State {
    pub wait_handle: Option<Mutex<JoinHandle<()>>>
    
}

impl State {
    pub fn new() -> State {
        State { wait_handle: None }
    }
}