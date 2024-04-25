//Etape 7
async function createNote() {
    const title = document.getElementById('title').value;
    const content = document.getElementById('content').value;
    await window.rpc.invoke('create_note', { title, content });
  }
  
  async function readNotes() {
    await window.rpc.invoke('get_notes').then(notes => {
        document.getElementById('notesread').innerText = notes;
    })
    .catch(err => alert('Error load notes: ' + err));
  }
  
  async function updateNote() {
    const id = document.getElementById('updateId').value;
    const title = document.getElementById('updateTitle').value;
    const content = document.getElementById('updateContent').value;
    await window.rpc.invoke('update_note', { id, title, content });
  }
  
  async function deleteNote() {
    const id = document.getElementById('deleteId').value;
    await window.rpc.invoke('delete_note', { id });
  }
  
  window.addEventListener('DOMContentLoaded', () => {
    document.getElementById('createNoteButton').addEventListener('click', createNote);
    document.getElementById('readNotesButton').addEventListener('click', readNotes);
    document.getElementById('updateNoteButton').addEventListener('click', updateNote);
    document.getElementById('deleteNoteButton').addEventListener('click', deleteNote);
  });
  