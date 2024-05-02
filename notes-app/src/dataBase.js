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
       const note = document.getElementById('notesread').innerText = notes;
        console.log('note',note);
    })
    .catch(err => alert('Error load notes: ' + err));
  }
  
  async function updateNote() {
    const id = parseInt(document.getElementById('updateId').value, 10);
    const title = document.getElementById('updateTitle').value;
    const content = document.getElementById('updateContent').value;
    console.log('updateNote() called ',title,content,id);
    await invoke('update_note', { id, title, content });
  }
  
  async function deleteNote() {
    const id = parseInt(document.getElementById('deleteId').value, 10);
    console.log('deleteNote() called ',id);
    await invoke('delete_note', { id });
  }
  async function exportNote(){
    const noteId = parseInt(document.getElementById('noteId').value, 10);
    if (!isNaN(noteId)) {
        window.__TAURI__.invoke('export_note_to_pdf', { id: noteId })
            .then(response => alert('Succès : ' + response))
            .catch(error => alert('Erreur : ' + error));
    } else {
        alert('Veuillez entrer un ID de note valide.');
    }
}
  async function exportAllNotes() {
    await invoke('export_all_notes_to_pdf')
      .then(response => {
          alert('Toutes les notes ont été exportées avec succès: ' + response);
      })
      .catch(error => {
          alert('Erreur lors de l\'exportation de toutes les notes en PDF: ' + error);
      });
  }
  async function searchNotes() {
    const query = document.getElementById('searchQuery').value;

    await invoke('search_notes', { query })
        .then(notes => {
            const resultsElement = document.getElementById('searchResults');
            resultsElement.innerHTML = '';
            console.log('notes',notes);
            if (notes.length === 0) {
                resultsElement.innerHTML = '<li>Aucune note trouvée.</li>';
            } else {
                notes.forEach(note => {
                    const li = document.createElement('li');
                    li.textContent = `Titre: ${note.title}, Contenu: ${note.content}`;
                    resultsElement.appendChild(li);
                });
            }
        })
        .catch(err => {
            alert('Erreur lors de la recherche de notes: ' + err);
        });
}

  window.addEventListener('DOMContentLoaded', () => {
    const createNoteButton = document.getElementById('createNoteButton');
    const readNotesButton = document.getElementById('readNotesButton');
    const updateNoteButton = document.getElementById('updateNoteButton');
    const deleteNoteButton = document.getElementById('deleteNoteButton');
    const exportAllPdfButton = document.getElementById('exportAllPdfButton');
    document.getElementById('searchButton').addEventListener('click', searchNotes);
    if(exportAllPdfButton){
      exportAllPdfButton.addEventListener('click', exportAllNotes);
    }
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


