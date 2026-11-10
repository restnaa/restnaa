import { useEffect, useState } from 'react'
import { getDashboardState, subscribeScheduler } from '../lib/api'
import type { DashboardState } from '../lib/types'

type SchedulerView = { status: 'loading' } | { status: 'ready'; state: DashboardState } | { status: 'error'; message: string }

export function useScheduler(): SchedulerView {
  const [view, setView] = useState<SchedulerView>({ status: 'loading' })
  useEffect(() => {
    let active = true
    let failed = false
    let unlisten: (() => void) | undefined
    const accept = (state: DashboardState) => {
      if (active && !failed) setView(current => current.status === 'ready' && current.state.revision > state.revision ? current : { status: 'ready', state })
    }
    // Subscribe first so a reminder emitted during the initial invoke is not lost.
    subscribeScheduler('scheduler-state-changed', event => {
      if (!active) return
      if (event.kind === 'error') {
        failed = true
        setView({ status: 'error', message: event.message })
      } else accept(event.state)
    }).then(stop => {
      if (!active) { stop(); return }
      unlisten = stop
      return getDashboardState().then(accept)
    }).catch(() => {
      if (active) { failed = true; setView({ status: 'error', message: 'Scheduler unavailable. Check the desktop connection and local storage.' }) }
    })
    return () => { active = false; unlisten?.() }
  }, [])
  return view
}
