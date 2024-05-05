// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Mutex, OnceLock}};

use app::{
    application::Application,
    config_manger::{ConfigManager, ConfigurationDto},
};
use serde::{Deserialize, Serialize};
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
    fn new_bool(result: bool) -> CommandExecutionResult<bool> {
        CommandExecutionResult {
            result: Some(result),
            error: None,
        }
    }
}
impl CommandExecutionResult<ConfigurationDto> {
    fn new_config(result: ConfigurationDto) -> CommandExecutionResult<ConfigurationDto> {
        CommandExecutionResult {
            result: Some(result),
            error: None,
        }
    }
    fn error(message: &str) -> CommandExecutionResult<ConfigurationDto> {
        CommandExecutionResult {
            result: None,
            error: Some(ExecutionError{message: message.to_string()}),
        }
    }    
}

#[tauri::command]
fn save_config(config: ConfigurationDto) -> CommandExecutionResult<bool> {
    app().lock().unwrap().config_manager.set(config);
    CommandExecutionResult::new_bool(true)
}

#[tauri::command]
fn get_config() -> CommandExecutionResult<ConfigurationDto> {
    let mut mg = app().lock().unwrap();    
    if let Some(config) =  mg.config_manager.get() {
        return CommandExecutionResult::new_config(config.clone());
    }
    return CommandExecutionResult::error("Конфигурации не существует");
}
