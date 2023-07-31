// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, collections::HashMap, sync::Arc, time::Duration, ops::{DerefMut, Deref}, borrow::BorrowMut, io, path::Path};

use album::{Album, AlbumMap};
use state::State;
use tauri::{async_runtime::{Mutex, self, spawn_blocking}, Manager};
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

fn mkdir_if_not_exists(path: &Path) -> Result<(), io::Error> {
    if !path.try_exists()? {
        fs::create_dir_all(path)?;
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    
    
    
    tauri::Builder::default()
        .setup(|app|{

            let app_data_dir  = app.path_resolver().app_data_dir().expect("Failed to query app data directory");
            let app_cache_dir = app.path_resolver().app_cache_dir().expect("Failed to query app cache directory");

            if let Err(e) = mkdir_if_not_exists(&app_data_dir) {
                println!("Failed to create app data directory {}: {}", app_data_dir.display(), e);
            }

            if let Err(e) = mkdir_if_not_exists(&app_cache_dir) {
                println!("Failed to create app cache directory {}: {}", app_cache_dir.display(), e);
            }

            let mut state = State::new();
            state.wait_handle = Some(Mutex::new(
                tauri::async_runtime::spawn(async move {
                    let album_path = app_data_dir.join("albums.toml");
                    match album::load_albums(&album_path).await {
                        Ok(album_map) => album_map,
                        Err(e) => {
                            println!("Error occured while loading albums: {e}");
                            AlbumMap::new()
                        }
                    }
                })
            ));
                
            app.manage(state);
            Ok(())      
        })
        .invoke_handler(tauri::generate_handler![greet, readdir, long_function])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
