// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, OnceLock};

use app::{
    application::Application,
    config_manger::{ConfigManager, ConfigurationDto},
};
use serde::{Deserialize, Serialize};

fn app() -> &'static Mutex<Application> {
    static APPLICATION: OnceLock<Mutex<Application>> = OnceLock::new();
    APPLICATION.get_or_init(|| Mutex::new(
        Application {
            config_manager: ConfigManager { current: None },
        }
    ))
}


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
    app().lock().unwrap().config_manager.set(config);

    Release {
        name: String::from("done"),
    }
}
