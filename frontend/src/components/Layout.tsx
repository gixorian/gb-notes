import { Outlet } from 'react-router-dom';
import { Navbar } from './Navbar';
import './Navbar.css'


export function Layout() {
  return (
    <>
      <Navbar />
      <main>
        <Outlet />
      </main>
    </>
  );
}
