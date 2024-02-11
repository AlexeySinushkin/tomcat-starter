// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::domain::config::ConfigurationDto;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub name: String,
}

#[tauri::command]
fn save_config(config: ConfigurationDto) -> Release {
    println!("configuration to save {:?}", config);
    Release {
        name: String::from("done"),
    }
}
