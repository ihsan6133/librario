// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};

use tauri::async_runtime::Mutex;

mod state;
mod album;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn readdir(path: &str, state: tauri::State<'_, state::State>) -> Result<Vec<String>, String> {
    
    let mut name = state.name.lock().await;
    println!("{}", name);
    
    *name = path.into();
    Ok(fs::read_dir(path).map_err(|e|e.to_string())?.filter_map(|e|{
        Some(e.ok()?.file_name().to_str()?.to_string())
    }).collect())
}
fn main() {
    
    tauri::Builder::default()
        .manage(state::State{name: Mutex::new("My librario".to_string())})
        .invoke_handler(tauri::generate_handler![greet, readdir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
