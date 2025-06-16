import { Link } from 'react-router-dom';

export function Navbar() {
  return (
    <>
      <nav>
        <Link to="/">Home</Link>
        <Link to="/about">About</Link>
        <Link to="/notes">My Notes</Link>
        <Link to="/notes/new">New Note</Link>
        <Link to="/account" className='push-right'>My Account</Link>
      </nav >
    </>
  );
}
