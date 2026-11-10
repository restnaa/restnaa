import { useEffect, useRef, useState } from 'react'
import { AppLayout } from './components/AppLayout'
import { getAppInfo, getSettings, subscribeScheduler } from './lib/api'
import type { AppInfoState, Page } from './lib/types'
import { Dashboard } from './pages/Dashboard'
import { Settings } from './pages/Settings'
import { History } from './pages/History'

export default function App() {
  const [page, setPage] = useState<Page>('dashboard')
  const [appInfo, setAppInfo] = useState<AppInfoState>({ status: 'loading' })
  const navigated = useRef(false)
  const navigate = (target: Page) => { navigated.current = true; setPage(target) }

  useEffect(() => {
    let active = true
    getAppInfo().then(
      info => { if (active) setAppInfo({ status: 'loaded', info }) },
      () => { if (active) setAppInfo({ status: 'error', message: 'Desktop connection unavailable. Open Restnaa through the desktop app to check local storage.' }) },
    )
    getSettings().then(settings => { if (active && !navigated.current && !settings.onboardingCompleted) setPage('settings') }).catch(() => { /* Connection/storage errors are rendered by the screens. */ })
    return () => { active = false }
  }, [])

  useEffect(() => {
    let active = true
    let stop: (() => void) | undefined
    subscribeScheduler('navigate', target => { if (active && ['dashboard','settings','history'].includes(target)) { navigated.current = true; setPage(target) } }).then(unlisten => {
      if (!active) unlisten()
      else stop = unlisten
    }).catch(() => { /* Browser preview has no native tray navigation. */ })
    return () => { active = false; stop?.() }
  }, [])

  return <AppLayout page={page} navigate={navigate}>
    {page === 'dashboard' && <Dashboard appInfo={appInfo} navigate={navigate} />}
    {page === 'settings' && <Settings appInfo={appInfo} onSetupComplete={() => navigate('dashboard')} />}
    {page === 'history' && <History />}
  </AppLayout>
}
