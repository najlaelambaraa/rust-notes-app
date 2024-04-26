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
    pub async fn create_note(app_state: tauri::State<'_, AppState>, title: String, content: String) -> Result<(), InvokeError> {
        println!("Title: {}", title);
    let conn = app_state.conn.lock();
    conn.await.execute(
    "INSERT INTO notes (title, content) VALUES (?1, ?2)",
    params![title, content],
    ).map_err(map_error)?;
    Ok(())
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
    pub async fn update_note(app_state: tauri::State<'_, AppState>, id: i32, title: String, content: String) -> Result<(), InvokeError> {
    let conn = app_state.conn.lock().await;
    conn.execute(
    "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
    params![title, content, id],
    ).map_err(map_error)?;
    Ok(())
    }

    #[tauri::command]
    pub async fn delete_note(app_state: tauri::State<'_, AppState>, id: i32) -> Result<(), InvokeError> {
    let conn = app_state.conn.lock().await;
    conn.execute(
    "DELETE FROM notes WHERE id = ?1",
    params![id],
    ).map_err(map_error)?;
    Ok(())
    }
