use super::*;
use rusqlite::{params, Error as RusqliteError};
use tauri::InvokeError;
use genpdf::{elements, fonts, style, Document, SimplePageDecorator};
use rusqlite::Connection;
use std::fs::File;
use rusqlite::OptionalExtension;
use genpdf::Element;
use dirs;

#[derive(Debug, Clone)]
struct Note {
    id: i32,
    title: String,
    content: String,
}

/// Crée une nouvelle note dans la base de données.
///
/// # Paramètres
/// - `title`: Le titre de la note.
/// - `content`: Le contenu de la note.
///
/// # Retour
/// Retourne `Ok(())` si la note a été créée avec succès, sinon renvoie une `InvokeError`.
#[tauri::command]
pub fn create_note(title: &str, content: &str) -> Result<(), InvokeError> {
    let mut conn = Connection::open("notes.db").map_err(|e| InvokeError::from(e.to_string()))?;

    let transaction = conn.transaction().map_err(|e| InvokeError::from(e.to_string()))?;

    transaction.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![title, content],
    ).map_err(|e| InvokeError::from(e.to_string()))?;

    transaction.execute(
        "INSERT INTO notes_fts (title, content) VALUES (?1, ?2)",
        params![title, content],
    ).map_err(|e| InvokeError::from(e.to_string()))?;

    transaction.commit().map_err(|e| InvokeError::from(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn get_note(id: i32) -> Result<Option<(String, String)>, InvokeError> {
    let conn = Connection::open("notes.db").expect("failed to open database");

    let mut stmt = conn.prepare("SELECT title, content FROM notes WHERE id = ?1").map_err(map_error)?;

    let note_result = stmt.query_row(params![id], |row| {
        Ok((row.get(0)?, row.get(1)?))
    });
    match note_result {
        Ok(note) => Ok(Some(note)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None), // Aucune note trouvée pour cet ID
        Err(e) => Err(map_error(e)), // Autre erreur de base de données
    }
}

fn map_error(err: rusqlite::Error) -> InvokeError {
    InvokeError::from(err.to_string())
}

#[tauri::command]
pub fn get_notes() -> Result<Vec<(i32, String, String)>, InvokeError> {
    let conn = Connection::open("notes.db").map_err(map_error)?;

    let mut stmt = conn.prepare("SELECT id, title, content FROM notes").map_err(map_error)?;

    let notes_iter = stmt.query_map(params![], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    }).map_err(map_error)?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note.map_err(map_error)?);
    }

    Ok(notes)
}

    #[tauri::command]
pub fn update_note(id: i32,title: &str, content: &str) -> Result<(), InvokeError> {
    let conn = Connection::open("notes.db").expect("failed to open database");
    conn.execute(
    "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
    params![title, content, id],
    ).map_err(map_error)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_note(id: i32) -> Result<(), InvokeError> {
    let conn = Connection::open("notes.db").expect("failed to open database");
    conn.execute(
    "DELETE FROM notes WHERE id = ?1",
    params![id],
    ).map_err(map_error)?;
    Ok(())
}
    
/// Exporte une note en fichier PDF
#[tauri::command]
pub fn export_note_to_pdf(id: i32) -> Result<String, String> {
    let conn = Connection::open("notes.db").map_err(|e| e.to_string())?;
    let note = conn.query_row(
        "SELECT title, content FROM notes WHERE id = ?1",
        params![id],
        |row| {
            let title: String = row.get(0)?;
            let content: String = row.get(1)?;
            Ok((title, content))
        }
    ).optional().map_err(|e| e.to_string())?;

    if let Some((title, content)) = note {
        let font_family = fonts::from_files("./fonts", "LiberationSans", None)
            .map_err(|e| format!("Failed to load font family: {}", e))?;
        let mut doc = Document::new(font_family);

        doc.set_page_decorator(SimplePageDecorator::new());

        let mut header_style = style::Style::new();
        header_style.set_font_size(20);
        let header = elements::Paragraph::new(&title).styled(header_style);
        doc.push(header);

        let mut text_style = style::Style::new();
        text_style.set_font_size(12);
        let text = elements::Paragraph::new(&content).styled(text_style);
        doc.push(text);

        let file_path = format!("{}-{}.pdf", id, title.replace(" ", "_"));
        let file = File::create(&file_path).map_err(|e| e.to_string())?;
        doc.render(file).map_err(|e| e.to_string())?;

        Ok(format!("Note exported to {}", file_path))
    } else {
        Err("Note not found".to_string())
    }
}

#[tauri::command]
pub fn export_all_notes_to_pdf() -> Result<String, String> {
    let downloads_dir = match dirs::download_dir() {
        Some(dir) => dir,
        None => return Err(String::from("Failed to get the downloads directory")),
    };
    let file_path = downloads_dir.join("notes.pdf");
    Ok(format!("All notes exported to {}", file_path.to_string_lossy()))      
}
 
// #[tauri::command]
// pub fn search_notes(conn: &Connection, query: &str) -> Result<Vec<(String, String)>, InvokeError> {
//     let mut stmt = conn.prepare("SELECT title, content FROM notes_fts WHERE notes_fts MATCH ?").map_err(|e| InvokeError::from(e.to_string()))?;
//     let notes_iter = stmt.query_map(params![query], |row| {
//         Ok((row.get(0)?, row.get(1)?))
//     }).map_err(|e| InvokeError::from(e.to_string()))?;

//     let mut notes = Vec::new();
//     for note in notes_iter {
//         notes.push(note.map_err(|e| InvokeError::from(e.to_string()))?);
//     }
//     Ok(notes)
// }

#[tauri::command]
pub fn search_notes(query: &str) -> Result<Vec<(i32, String, String)>, InvokeError> {
    let conn = Connection::open("notes.db").map_err(map_error)?;
    let mut stmt = conn.prepare("SELECT id, title, content FROM notes WHERE title LIKE ?1 OR content LIKE ?1")
        .map_err(map_error)?;

    let query = format!("%{}%", query); 
    let notes_iter = stmt.query_map(params![query], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    }).map_err(map_error)?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note.map_err(map_error)?);
    }

    Ok(notes)
}
