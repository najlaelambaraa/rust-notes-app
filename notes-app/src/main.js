const { invoke } = window.__TAURI__.tauri;

async function greet() {
  const greetInputEl = document.getElementById('greet-input');
  const greetMsgEl = document.getElementById('greet-msg');
  greetMsgEl.textContent = await invoke('greet', { name: greetInputEl.value });
}
//Etape 5
async function saveNote() {
  const note = document.getElementById('noteInput').value;
  await invoke('save_note', { note })
      .then(() => {
          alert('Note saved!');
          document.getElementById('noteInput').value = ''; 
          loadNotes(); 
      })
      .catch(err => alert('Error to save note: ' + err));
}

async function loadNotes() {
  await invoke('read_notes')
      .then(notes => {
          document.getElementById('notesDisplay').innerText = notes;
      })
      .catch(err => alert('Error load notes: ' + err));
}

async function update_Note() {
  const noteId = document.getElementById('noteId').value;
  const newContent = document.getElementById('newContent').value;
  console.log('updateNote() called ',newContent,noteId);
  await invoke('update_file_note', { noteId, newContent })
    .then(() => {
      alert('Note updated!');
      document.getElementById('noteId').value = '';
      document.getElementById('newContent').value = '';
      loadNotes();
    })
    .catch(err => alert('Error to update note: ' + err));
}

async function delete_Note() {
  const noteId = document.getElementById('deleteNoteId').value;
  await invoke('delete_file_note', { noteId })
      .then(() => {
          alert('Note deleted');
          document.getElementById('deleteNoteId').value = '';
          loadNotes();
      })
      .catch(err => alert('Error to delete note: ' + err));
}


window.addEventListener('DOMContentLoaded', () => {
  document.getElementById('saveNoteBtn').addEventListener('click', saveNote);
  document.getElementById('loadNotesBtn').addEventListener('click', loadNotes);
  document.getElementById('updateNoteBtn').addEventListener('click', update_Note); 
  document.getElementById('deleteNoteBtn').addEventListener('click',function(event) {
    event.preventDefault();
    delete_Note()
});
});


