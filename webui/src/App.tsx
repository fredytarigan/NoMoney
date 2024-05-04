import { useState } from 'react'
import { Button } from './components/ui/button'
import './App.css'
import { Navigate, Route, Routes } from 'react-router-dom'
import Dashboard from './pages/Dashboard'
import Login from './pages/Login'

function App() {
  return (
    <>
      <Routes>
        <Route path="/" element={ <Navigate to="/dashboard" /> } />
        <Route path="/dashboard" element={ <Dashboard /> } />
        <Route path="/login" element={ <Login /> } />
      </Routes>
    </>
  )
}

export default App
