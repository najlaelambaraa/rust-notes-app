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
  console.log('updateNote() called');
  const oldNote = document.getElementById('oldNote').value;
  const newNote = document.getElementById('newNote').value;
  await invoke('update_file_note', { oldNote, newNote })
      .then(() => {
          alert('Note updated!');
          document.getElementById('oldNote').value = '';
          document.getElementById('newNote').value = '';
          loadNotes();
      })
      .catch(err => alert('Error to update note: ' + err));
}

async function delete_Note() {
  const note = document.getElementById('deleteNote').value;
  await invoke('delete_file_note', { note })
      .then(() => {
          alert('Note deleted');
          document.getElementById('deleteNote').value = '';
          loadNotes();
      })
      .catch(err => alert('Error to delete note: ' + err));
}

window.addEventListener('DOMContentLoaded', () => {
  document.getElementById('saveNoteBtn').addEventListener('click', saveNote);
  document.getElementById('loadNotesBtn').addEventListener('click', loadNotes);
  document.getElementById('greet-form').addEventListener('submit', (e) => {
    e.preventDefault();
    greet();
  });
  document.getElementById('updateNoteBtn').addEventListener('click', update_Note); 
  document.getElementById('deleteNoteBtn').addEventListener('click', delete_Note);
});


