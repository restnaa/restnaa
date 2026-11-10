import { useCallback, useEffect, useRef, useState } from 'react'
import { getHistory, subscribeScheduler } from '../lib/api'
import type { HistoryRange, LocalHistory } from '../lib/types'

type View = { status: 'loading' } | { status: 'ready'; history: LocalHistory } | { status: 'error'; message: string }
export function useHistory(range: HistoryRange) {
  const [view, setView] = useState<View>({ status: 'loading' })
  const refreshRef = useRef<() => void>(() => {})
  const refresh = useCallback(() => refreshRef.current(), [])
  useEffect(() => {
    let active = true
    let generation = 0
    const stops: (() => void)[] = []
    const load = () => {
      const current = ++generation
      setView({ status: 'loading' })
      void getHistory(range).then(history => {
        if (active && current === generation) setView({ status: 'ready', history })
      }, () => { if (active && current === generation) setView({ status: 'error', message: 'Local history unavailable. Check the desktop connection and local storage.' }) })
    }
    refreshRef.current = load
    const onVisible = () => { if (document.visibilityState === 'visible') load() }
    window.addEventListener('focus', load)
    document.addEventListener('visibilitychange', onVisible)
    // Subscribe before reading. Concurrent refreshes accept only the newest reply.
    void Promise.all((['break-event-recorded', 'history-changed'] as const).map(name =>
      subscribeScheduler(name, () => { if (active) load() }).then(stop => { if (active) stops.push(stop); else stop() })
    )).then(() => { if (active) load() }).catch(() => { if (active) setView({ status: 'error', message: 'Local history unavailable. Check the desktop connection and local storage.' }) })
    return () => { active = false; generation++; stops.forEach(stop => stop()); window.removeEventListener('focus', load); document.removeEventListener('visibilitychange', onVisible); refreshRef.current = () => {} }
  }, [range])
  // Never display a previous period's data while its replacement is loading.
  return { view: view.status === 'ready' && view.history.range !== range ? { status: 'loading' } as const : view, refresh }
}
