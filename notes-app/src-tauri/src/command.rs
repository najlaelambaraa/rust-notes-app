    use tauri::State;
    use async_trait::async_trait;
    // use serde::{Deserialize, Serialize};
    use tauri::command;
    use super::*;
    use std::sync::{Arc, Mutex};
    use serde::{ser::Serializer, Deserialize, Serialize};
    use rusqlite::{params, Error as RusqliteError};
    // use rusqlite::{params, Result as SqlResult};
    // use thiserror::Error;
    // use anyhow::Result;
    use tauri::InvokeError;
    use std::env::Args;
   
    //Etape 7
    #[derive(Debug, Clone)]
    struct Note {
        id: i32,
        title: String,
        content: String,
    }
    // #[command]
    // pub fn add_note(conn: &Connection, note: &str) {
    // conn.execute("INSERT INTO notes (content) VALUES (?1);", &[note]).expect("Failed to add note");
    // }

    // trait CommandKind {
    //     fn blocking_kind(&self) -> bool;
    // }
    
    // impl CommandKind for Result<Vec<(i32, String, String)>, MyError> {
    //     fn blocking_kind(&self) -> bool {
    //         // Determine if the operation is blocking
    //         true
    //     }
    // }
    
//     #[command]
//     pub async fn get_notes(ctx: &Context, msg: &Message, args: Args) -> Result<Vec<Note>,InvokeError>{
//     let conn = Connection::open("notes.db").expect(InvokeError);

//     let mut stmt = conn.prepare("SELECT id, title, content FROM notes")?;
//     let notes_iter = stmt.query_map([], |row| {
//         Ok((
//             row.get(0)?,
//             row.get(1)?,
//             row.get(2)?,
//         ))
//     })?;

//     let mut notes = Vec::new();
//     for note_result in notes_iter {
//         notes.push(note_result?);
//     }

//     Ok(notes)
// }
#[tauri::command]
pub fn create_note(title: &str, content: &str) -> Result<(),InvokeError> {
    let conn = Connection::open("notes.db").expect("failed to open database");
    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![title, content],
    ).expect("failed to insert note");
    Ok(())
}
#[tauri::command]
pub fn get_note(id: i32) -> Result<Option<(String, String)>, InvokeError> {
    let conn = Connection::open("notes.db").expect("failed to open database");

    // Prépare une requête pour sélectionner la note
    let mut stmt = conn.prepare("SELECT title, content FROM notes WHERE id = ?1").map_err(map_error)?;

    // Exécute la requête et récupère la première ligne comme résultat
    let note_result = stmt.query_row(params![id], |row| {
        Ok((row.get(0)?, row.get(1)?))
    });

    // Gérer le résultat ou l'absence de celui-ci
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

    // #[tauri::command]
    // pub async fn get_notes(app_state: tauri::State<'_, AppState>) -> Result<Vec<Note>, InvokeError> {
    // let conn = app_state.conn.lock().await;
    // let mut stmt = conn.prepare("SELECT id, title, content FROM notes").map_err(map_error)?;
    
    // let note_iter = stmt.query_map([], |row| {
    //     Ok(Note {
    //         id: row.get(0)?,
    //         title: row.get(1)?,
    //         content: row.get(2)?,
    //     })
    // }).map_err(map_error)?;
    
    // let mut notes = Vec::new();
    // for note in note_iter {
    //     notes.push(note.map_err(map_error)?);
    // }
    // Ok(notes)
    
    // }

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
