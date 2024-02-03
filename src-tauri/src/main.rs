// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use std::fmt;




fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub name: String,
}


#[tauri::command]
fn greet(r : Release) -> Release {
    Release {
        name: r.name.clone(),
    }
}

type Result<T> = std::result::Result<T, DoubleError>;
#[derive(Debug, Clone)]
struct DoubleError;
impl fmt::Display for DoubleError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "invalid first item to double")
  }
}

#[tauri::command]
fn greet_err(_r : Release) -> Result<Release> {
  Err(DoubleError{})
}
