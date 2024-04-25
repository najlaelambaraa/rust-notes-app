// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::Write;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
//Etape 5
#[tauri::command]
fn save_note(note: String) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("failed to open file");
        writeln!(file, "{}", note).expect("failed to write note");
}


//Lire des notes
#[tauri::command]
fn read_notes() -> String {
    std::fs::read_to_string("notes.txt").unwrap_or_else(|_| "".to_string())
}
//Mettons à jour les notes
#[tauri::command]
fn update_note(old_note: String, new_note: String) -> Result<(), String> {
    let contents = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
    let new_contents = contents.replace(&old_note, &new_note);
    std::fs::write("notes.txt", new_contents).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_note(note: String) -> Result<(), String> {
    let contents = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
    let new_contents = contents.replace(&format!("{}\n", note), "");
    std::fs::write("notes.txt", new_contents).map_err(|e| e.to_string())
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_note, read_notes, update_note, delete_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
