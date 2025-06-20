import { useEffect, useState } from 'react';
import './Notes.css';

function Notes() {

  type Note = {
    id: number;
    title: string;
    content: string;
    created_at: string;
  }

  const [notes, setNotes] = useState<Note[]>([]);



  useEffect(() => {
    fetch('/api/notes')
      .then(res => res.json())
      .then(data => setNotes(data))
      .catch(err => console.error('Error fetching notes:', err));
  }, []);

  return (
    <>
      <div>
        <h1>My Notes</h1>
      </div>
      <div className="notes">
        {notes.map(note => (
          <div key={note.id} className="note">
            <header className="note-title">
              <h3>{note.title}</h3>
            </header>
            <h4 className="note-content">{note.content}</h4>
            <footer className="note-date">
              <h5> {' '}
                {new Date(note.created_at).toLocaleString('en-US', {
                  dateStyle: 'medium',
                  timeStyle: 'short'
                })}
              </h5>
            </footer>
          </div>
        ))}
      </div >
    </>
  );
}

export default Notes;
