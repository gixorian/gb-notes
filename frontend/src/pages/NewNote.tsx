import './NewNote.css';
import React, { useState, useEffect } from 'react';

function NewNote() {

  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [status, setStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle');
  const [showNotification, setShowNotification] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatus('saving');

    try {
      const res = await fetch('/api/new-note', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title, content })
      });

      if (!res.ok) throw new Error(`Server error: ${res.status} ${res.statusText}`);

      setStatus('saved');
      setTitle('');
      setContent('');
    } catch (err) {

      console.error(err);
      setStatus('error');
    }
  };

  useEffect(() => {
    if (status === 'saved' || status === 'error') {
      setShowNotification(true);
      const hideTimeout = setTimeout(() => {
        setShowNotification(false);
      }, 3000); // Show notification for 3 seconds

      const removeTimeout = setTimeout(() => {
        setStatus('idle'); // Reset status after notification
      }, 3500); // Reset status after 3.5 seconds

      return () => {
        clearTimeout(hideTimeout);
        clearTimeout(removeTimeout);
      };
    }
  }, [status]);

  return (
    <>
      <h1>New Note</h1>
      <form onSubmit={handleSubmit} className="new-note-form">
        <center>
          <input
            name="title"
            value={title}
            onChange={e => setTitle(e.target.value)}
            type="text" placeholder="Title"
            size={100}
            required
          />
          <textarea
            name="note"
            value={content}
            onChange={e => setContent(e.target.value)}
            placeholder="Write your note here..."
            rows={10} cols={100}
            required>
          </textarea>
          <br />
          <div className="buttons">
            <button id="cancel" type="button">Cancel</button>
            <button id="save" type="submit">
              {/* <button id="save" type="submit" disabled={status === 'saving'}> */}
              {/* {status === 'saving' ? 'Saving...' : "Save"} */}
              Save
            </button>
            {status === 'saved' && <div className={`notification saved ${showNotification ? 'show' : ''}`}>Note saved!</div>}
            {status === 'error' && <div className={`notification error ${showNotification ? 'show' : ''}`}>Error saving the note!</div>}
          </div>
        </center>
      </form>
    </>
  );
}

export default NewNote;
