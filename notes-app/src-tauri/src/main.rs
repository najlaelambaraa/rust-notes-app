#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use rusqlite::{params, Connection, Result};
use rusqlite::{Connection, Result as SqliteResult};
use std::sync::Arc;
use tauri::{self, InvokeError};
use tokio::sync::Mutex;
mod command;
use command::{create_note, update_note, delete_note,get_notes,export_note_to_pdf,export_all_notes_to_pdf,search_notes};
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
fn create_fts_table(conn: &Connection) -> Result<(), rusqlite::Error> {

    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(title, content);",
        [],
    )?;
    Ok(())
}

struct AppState {
    conn: Arc<Mutex<Connection>>,
}

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("notes.db").expect("failed to open database");
    create_fts_table(&conn)?;

    let app_state = Arc::new(Mutex::new(AppState { conn: Arc::new(Mutex::new(conn)) }));
    tauri::Builder::default()
        .manage(app_state) 
        .invoke_handler(tauri::generate_handler![export_all_notes_to_pdf,export_note_to_pdf,get_notes,create_note,update_note, delete_note,read_notes,save_note,update_file_note,delete_file_note,search_notes])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
    Ok(())
}


