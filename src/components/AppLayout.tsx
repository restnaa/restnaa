import { useEffect, useRef, type ReactNode } from 'react'
import { House, Settings, History, Pause } from 'lucide-react'
import type { Page } from '../lib/types'
const pages = [
  { id: 'dashboard', label: 'Dashboard', icon: House },
  { id: 'settings', label: 'Settings', icon: Settings },
  { id: 'history', label: 'History', icon: History },
] as const
export function AppLayout({ page, navigate, children }: { page: Page; navigate: (page: Page) => void; children: ReactNode }) {
  const mainRef = useRef<HTMLElement>(null)
  const previousPage = useRef(page)
  useEffect(() => {
    if (previousPage.current !== page) mainRef.current?.focus()
    previousPage.current = page
  }, [page])
  return <div className="app-shell">
    <a className="skip-link" href="#main-content">Skip to content</a>
    <header className="app-header">
      <div className="brand"><span className="brand-mark"><Pause size={23} fill="currentColor" strokeWidth={0} aria-hidden="true" /></span><span>Restna</span></div>
      <nav aria-label="Main navigation">{pages.map(({ id, label, icon: Icon }) => <button key={id} type="button" aria-current={page === id ? 'page' : undefined} onClick={() => navigate(id)}><Icon size={22} strokeWidth={1.8} aria-hidden="true" /><span>{label}</span></button>)}</nav>
    </header>
    <main id="main-content" ref={mainRef} tabIndex={-1} className="workspace">{children}</main>
  </div>
}
