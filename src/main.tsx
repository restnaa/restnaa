import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'
import './styles.css'
import { waitForDesktopReady } from './lib/startup'
import { BreakPanel } from './pages/BreakPanel'

const isBreakWindow = new URLSearchParams(window.location.search).get('window') === 'break'
const root = createRoot(document.getElementById('root')!)
root.render(<main className="break-panel"><p role="status">Starting Restnaa…</p></main>)
void waitForDesktopReady().finally(() => {
  root.render(<StrictMode>{isBreakWindow ? <BreakPanel /> : <App />}</StrictMode>)
})
