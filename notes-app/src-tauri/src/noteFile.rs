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

#[tauri::command]
pub fn update_file_note(note_id: String, new_content: String) -> Result<(), String> {
    println!("note_id: {}", note_id);
    let note_id: usize = note_id.parse().map_err(|_| "Invalid note ID".to_string())?;
    let mut contents = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
    let mut lines: Vec<&str> = contents.lines().collect();
    if note_id >= lines.len() {
        return Err("Note ID out of range".to_string());
    }
    lines[note_id] = &new_content;
    let new_contents = lines.join("\n");
    std::fs::write("notes.txt", new_contents).map_err(|e| e.to_string())
}

//Delete notes
#[tauri::command]
pub fn delete_file_note(note_id: String) -> Result<(), String> {
    let note_id: usize = note_id.parse().map_err(|_| "Invalid note ID".to_string())?;
    let contents = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
    let mut lines: Vec<String> = contents.lines().map(String::from).collect();
    if note_id < lines.len() {
        lines.remove(note_id);
        let new_contents = lines.join("\n");
        std::fs::write("notes.txt", new_contents).map_err(|e| e.to_string())
    } else {
        Err("Note ID out of range".to_string())
    }
}
