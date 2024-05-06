import './App.css'
import AppProvider from './providers/apps'
import PrivateRoutes from './routes/Private'
import PublicRoutes from './routes/Public'

function App() {
  return (
    <AppProvider>
      <PublicRoutes />
      <PrivateRoutes />
    </AppProvider>
  )
}

export default App
