import { Routes, Route } from 'react-router-dom'
import { Layout } from './components/Layout'
import Home from './pages/Home'
import About from './pages/About'
import Notes from './pages/Notes'
import NewNote from './pages/NewNote'
import Account from './pages/Account'
import './App.css'

export default function App() {

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/" element={<Home />} />
        <Route path="/about" element={<About />} />
        <Route path="/notes" element={<Notes />} />
        <Route path="/notes/new" element={<NewNote />} />
        <Route path="/account" element={<Account />} />
      </Route>
    </Routes>
  );
}
