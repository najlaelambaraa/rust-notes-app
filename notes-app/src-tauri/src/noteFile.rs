//Etape 5
use std::io::Write;

#[tauri::command]
pub fn save_note(note: String) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("failed to open file");
        writeln!(file, "{}", note).expect("failed to write note");
}

//Read notes
#[tauri::command]
pub fn read_notes() -> String {
    std::fs::read_to_string("notes.txt").unwrap_or_else(|_| "".to_string())
}

//Update notes
#[tauri::command]
pub fn update_file_note(old_note: String, new_note: String) -> Result<(), String> {
    let contents = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
    let new_contents = contents.replace(&old_note, &new_note);
    std::fs::write("notes.txt", new_contents).map_err(|e| e.to_string())
}

//Delete notes
#[tauri::command]
pub fn delete_file_note(note: String) -> Result<(), String> {
    let contents = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
    let new_contents = contents.replace(&format!("{}\n", note), "");
    std::fs::write("notes.txt", new_contents).map_err(|e| e.to_string())
}
