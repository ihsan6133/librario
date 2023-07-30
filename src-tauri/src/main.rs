// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, collections::HashMap, sync::Arc, time::Duration, ops::{DerefMut, Deref}};

use album::{Album, AlbumMap};
use state::State;
use tauri::async_runtime::{Mutex, self};
use uuid::Uuid;

mod album;
mod state;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
async fn readdir(path: &str) -> Result<Vec<String>, String> {
    Ok(fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(|e| Some(e.ok()?.file_name().to_str()?.to_string()))
        .collect())
}

#[tauri::command]
async fn long_function(state: tauri::State<'_, State>) -> Result<AlbumMap, String> {
    if  let Some(album_map) = state.album_map.lock().await.deref() {
        return Ok(album_map.clone())
    } 


    let mutex = state.wait_handle.as_ref().unwrap();
    let mut guard = mutex.lock().await;
    let future = guard.deref_mut();

    let (album_map,) = tokio::join!(future);
    let album_map = album_map.expect("Failed to unwrap album_map");

    let mut state_map = state.album_map.lock().await;
    
    let clone = album_map.clone();
    
    *state_map = Some(album_map);


    Ok(clone)
}

#[tokio::main]
async fn main() {
    
    let mut state = State::new();
    
    {
        state.wait_handle = Some(Mutex::new(
            tauri::async_runtime::spawn(async{
                std::thread::sleep(Duration::from_secs(10));
                println!("Finished!");
                let mut map = AlbumMap::new();
                map.insert(Uuid::new_v4().to_string(), Album::new("Georgia"));
                map.insert(Uuid::new_v4().to_string(), Album::new("USA"));
                map.insert(Uuid::new_v4().to_string(), Album::new("Holidays"));
                map.insert(Uuid::new_v4().to_string(), Album::new("Paris"));
                map
            })
        ))
    }

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![greet, readdir, long_function])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
