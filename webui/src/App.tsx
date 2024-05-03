import { DarkThemeToggle } from "flowbite-react";
import { Routes, Route } from "react-router-dom";
import Dashboard from "./pages/Dashboard";
import Login from './pages/Login';
import './App.css';


function App() {

  return (
    <>
      
      <Routes>
        <Route path="/" element={ <Dashboard /> } />
        <Route path="/login" element={ <Login /> } />
      </Routes>
    </>
  );
}

export default App;
