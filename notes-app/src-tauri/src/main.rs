#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::Write;
//use rusqlite::{params, Connection, Result};
use rusqlite::{Connection, params, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{State,self, command as other_command, InvokeError, Manager};
use serde_json::json;
use tokio::sync::Mutex;
use async_trait::async_trait;
mod command;
use command::{create_note, update_note, delete_note,get_notes};
mod noteFile;
use noteFile::{save_note, read_notes,update_file_note,delete_file_note};


fn map_error<E>(err: E) -> InvokeError
where
    E: std::fmt::Display,
{
    InvokeError::from(err.to_string())
}

//Etape 6
// Database initialization function
fn init_db() -> SqliteResult<()> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL
        )", [],
    )?;
    Ok(())
}

struct AppState {
    conn: Arc<Mutex<Connection>>,
}

fn main() {
    let conn = Connection::open("notes.db").expect("failed to open database");
    let app_state = Arc::new(Mutex::new(AppState { conn: Arc::new(Mutex::new(conn)) }));

    tauri::Builder::default()
        .manage(app_state) 
        .invoke_handler(tauri::generate_handler![get_notes,create_note,update_note, delete_note,read_notes,save_note,update_file_note,delete_file_note])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}


