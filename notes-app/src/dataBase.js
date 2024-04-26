//Etape 7
const { invoke } = window.__TAURI__.tauri;

async function createNote() {
  console.log('createNote() called');
    const title = document.getElementById('title').value;
    const content = document.getElementById('content').value;
    console.log('content title', content , title);
    await invoke('create_note', { title, content }).then(() => {
      console.log('createNote Saved!');
      alert('Note saved!');
      document.getElementById('title').value = ''; 
      document.getElementById('content').value = '';  
  })
  .catch(err => alert('Error to save note: ' + err));
  console.log('createNote() end');

  }
  
  async function readNotes() {
    await invoke('get_notes').then(notes => {
        alert('Note saved!');
        document.getElementById('notesread').innerText = notes;
    })
    .catch(err => alert('Error load notes: ' + err));
  }
  
  async function updateNote() {
    const id = document.getElementById('updateId').value;
    const title = document.getElementById('updateTitle').value;
    const content = document.getElementById('updateContent').value;
    console.log('updateNote() called ',title,content,id);
    await invoke('update_note', { id, title, content });
  }
  
  async function deleteNote() {
    const id = document.getElementById('deleteId').value;
    console.log('deleteNote() called ',id);
    await invoke('delete_note', { id });
  }
  
  window.addEventListener('DOMContentLoaded', () => {
    const createNoteButton = document.getElementById('createNoteButton');
    const readNotesButton = document.getElementById('readNotesButton');
    const updateNoteButton = document.getElementById('updateNoteButton');
    const deleteNoteButton = document.getElementById('deleteNoteButton');

    if (createNoteButton) {
        createNoteButton.addEventListener('click', createNote);
    }

    if (readNotesButton) {
        readNotesButton.addEventListener('click', readNotes);
    }

    if (updateNoteButton) {
        updateNoteButton.addEventListener('click', updateNote);
    }

    if (deleteNoteButton) {
        deleteNoteButton.addEventListener('click', deleteNote);
    }
});
