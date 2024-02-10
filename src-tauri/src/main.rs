// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, OnceLock};

use app::{
    application::Application,
    config_manger::{ConfigManager, ConfigurationDto},
};
use serde::{Deserialize, Serialize};
use serde_json::error;
static APPLICATION: OnceLock<Mutex<Application>> = OnceLock::new();
fn app() -> &'static Mutex<Application> {    
    APPLICATION.get_or_init(|| {
        Mutex::new(Application {
            config_manager: ConfigManager { current: None },
        })
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_config, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandExecutionResult<R> {
    pub result: Option<R>,
    pub error: Option<ExecutionError>,
}
impl CommandExecutionResult<bool> {
    fn newBool(result: bool) -> CommandExecutionResult<bool> {
        CommandExecutionResult {
            result: Some(result),
            error: None,
        }
    }
}
impl CommandExecutionResult<ConfigurationDto> {
    fn newConfig(result: ConfigurationDto) -> CommandExecutionResult<ConfigurationDto> {
        CommandExecutionResult {
            result: Some(result),
            error: None,
        }
    }
}

#[tauri::command]
fn save_config(config: ConfigurationDto) -> CommandExecutionResult<bool> {
    app().lock().unwrap().config_manager.set(config);
    CommandExecutionResult::newBool(true)
}

#[tauri::command]
fn get_config() -> CommandExecutionResult<ConfigurationDto> {
    let mut mg = app().lock().unwrap();
    return CommandExecutionResult::newConfig(mg.config_manager.get().clone());
}
