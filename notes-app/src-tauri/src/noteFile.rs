use std::io::Write;
use std::fs::File;
/// Enregistre une note dans un fichier texte.
/// 
/// Cette fonction ajoute une note à la fin d'un fichier `notes.txt`. Si le fichier n'existe pas,
/// il sera créé.
///
/// # Arguments
/// * `note` - Une chaîne de caractères contenant la note à enregistrer.
///
#[tauri::command]
pub fn save_note(note: String) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("failed to open file");
        writeln!(file, "{}", note).expect("failed to write note");
}

/// Lit et retourne toutes les notes du fichier `notes.txt`.
///
/// Si le fichier ne peut pas être lu, la fonction retourne une chaîne vide.
///
/// # Returns
/// Une chaîne de caractères contenant toutes les notes.
#[tauri::command]
pub fn read_notes() -> String {
    std::fs::read_to_string("notes.txt").unwrap_or_else(|_| "".to_string())
}

/// Met à jour le contenu d'une note spécifique dans le fichier `notes.txt`.
///
/// # Arguments
/// * `note_id` - L'identifiant de la note à mettre à jour, exprimé en chaîne de caractères.
/// * `new_content` - Le nouveau contenu de la note.
///
/// # Returns
/// Un `Result<(), String>` qui est `Ok` si la mise à jour a réussi, ou `Err` avec un message d'erreur.
///
/// # Errors
/// Renvoie une erreur si l'ID de la note est invalide ou si l'ID spécifié est hors limites.
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

/// Supprime une note spécifique du fichier `notes.txt`.
///
/// # Arguments
/// * `note_id` - L'identifiant de la note à supprimer, exprimé en chaîne de caractères.
///
/// # Returns
/// Un `Result<(), String>` qui est `Ok` si la suppression a réussi, ou `Err` avec un message d'erreur.
///
/// # Errors
/// Renvoie une erreur si l'ID de la note est invalide ou si l'ID spécifié est hors limites.
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
use genpdf::{Document, elements};
use genpdf::fonts::FontFamily;
use tauri::command;
use dirs;
// #[tauri::command]
// pub fn export_notes_to_pdf() -> Result<String, String> {
//     let notes = std::fs::read_to_string("notes.txt").map_err(|e| e.to_string())?;
//     let mut doc = Document::new(FontFamily::Helvetica);
//     doc.set_title("Mes notes");

//     doc.push(elements::Paragraph::new(&notes));

//     let downloads_dir = match dirs::download_dir() {
//         Some(path) => path,
//         None => return Err("Unable to find the downloads directory.".to_string()),
//     };

//     let output_path = downloads_dir.join("MyNotes.pdf");

//     doc.render_to_file(&output_path).map_err(|e| e.to_string())?;

//     Ok(output_path.to_str().unwrap_or_default().to_string())
// }

// #[tauri::command]
// pub fn export_all_notes_to_pdf() -> Result<String, String> {
//     let conn = Connection::open("notes.db").map_err(|e| e.to_string())?;
//     let mut stmt = conn.prepare("SELECT title, content FROM notes").map_err(|e| e.to_string())?;
//     let notes_iter = stmt.query_map(params![], |row| {
//         Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
//     }).map_err(|e| e.to_string())?;

//     let font_family = fonts::from_files("./src/static", "OpenSans", None)
//         .map_err(|e| format!("Failed to load font family: {}", e))?;
//     let mut doc = Document::new(font_family);
//     doc.set_page_decorator(SimplePageDecorator::new());

//     for note in notes_iter {
//         let (title, content) = note.map_err(|e| e.to_string())?;
//         let mut header_style = style::Style::new();
//         header_style.set_font_size(18);
//         let header = elements::Paragraph::new(&title).styled(header_style);
//         doc.push(header);

//         let mut text_style = style::Style::new();
//         text_style.set_font_size(12);
//         let text = elements::Paragraph::new(&content).styled(text_style);
//         doc.push(text);
//     }

//     let file_path = "all_notes.pdf";
//     let file = File::create(file_path).map_err(|e| e.to_string())?;
//     doc.render(file).map_err(|e| e.to_string())?;

//     Ok(format!("All notes exported to {}", file_path))
// }