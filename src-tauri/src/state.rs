//! Global State stored by Librario

use tauri::async_runtime::Mutex;

pub struct State {
    pub name: Mutex<String>
}