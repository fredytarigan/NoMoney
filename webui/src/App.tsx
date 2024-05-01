import { useEffect, useState } from 'react';
import { Routes, Route, BrowserRouter } from "react-router-dom";
import Dashboard from "./pages/Dashboard";
import { Person } from "./types";
import logo from './logo.svg';
import './App.css';


function App() {

  return (
    <>
      <BrowserRouter basename={process.env.PUBLIC_URL}>
        <Routes>
          <Route path="/" element={ <Dashboard /> } />
        </Routes>
      </BrowserRouter>
    </>
  );
}

export default App;
