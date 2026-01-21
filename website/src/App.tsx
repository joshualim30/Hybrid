import { BrowserRouter, Routes, Route } from 'react-router-dom'
import Layout from './components/Layout'
import Home from './pages/Home'
import Docs from './pages/Docs'
import Languages from './pages/Languages'
import Download from './pages/Download'

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={
            <>
              <Home />
            </>
          } />
          <Route path="languages" element={<Languages />} />
          <Route path="docs" element={<Docs />} />
          <Route path="download" element={<Download />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
