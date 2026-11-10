import { useEffect, useState } from 'react'
import { useScheduler } from '../hooks/useScheduler'
import { completeBreak, hideBreakWindow, skipBreak, snoozeBreak, startBreak } from '../lib/api'

export function BreakPanel() {
  const view = useScheduler()
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const current = view.status === 'ready' ? view.state.openReminder : null
  const finish = async (action: () => Promise<unknown>) => {
    await action()
    try { await hideBreakWindow() }
    catch { throw 'Your action was saved, but the window could not hide. Try Close for now.' }
  }
  const run = async (action: () => Promise<unknown>) => {
    if (busy) return
    setBusy(true); setError(null)
    try { await action() }
    catch (reason) { setError(typeof reason === 'string' ? reason : 'This break could not be updated. It may have expired; check the tray.') }
    finally { setBusy(false) }
  }
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !busy) {
        event.preventDefault()
        void hideBreakWindow().catch(() => setError('Could not hide the break window. Use its close button.'))
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [busy])
  return <main className="break-panel" aria-busy={busy}>
    <p className="break-brand">Restna</p>
    <h1>{current ? current.category === 'eye' ? 'A moment for your eyes.' : 'Make room to move.' : 'No break waiting.'}</h1>
    {view.status === 'loading' && <p role="status">Checking your current break…</p>}
    {view.status === 'error' && <p role="alert" className="error-message">{view.message}</p>}
    {error && <p role="alert" className="error-message">{error}</p>}
    {current && <>
      <p>{current.category === 'eye' ? 'Try looking about 20 feet away for 20 seconds.' : 'Try standing, walking briefly, or rolling your shoulders for a couple of minutes.'}</p>
      {current.startedAtUtc && <p role="status">Break started. Choose Done when you are ready.</p>}
      <div className="break-actions" role="group" aria-label="Break actions">
        <button type="button" className="secondary-button" disabled={busy || !!current.startedAtUtc} onClick={() => void run(() => startBreak(current.category))}>{current.startedAtUtc ? 'Started' : current.category === 'eye' ? 'Start 20-second break' : 'Start 2-minute break'}</button>
        <button type="button" className="primary-button" disabled={busy} onClick={() => void run(() => finish(() => completeBreak(current.id)))}>Done</button>
        <button type="button" className="secondary-button" disabled={busy} onClick={() => void run(() => finish(() => snoozeBreak(current.id)))}>Snooze 5 min</button>
        <button type="button" className="secondary-button" disabled={busy} onClick={() => void run(() => finish(() => skipBreak(current.id)))}>Skip</button>
      </div>
      <p className="break-note">Closing leaves this reminder unanswered until it times out. Only Done marks it complete.</p>
    </>}
    <button type="button" className="text-button" disabled={busy} onClick={() => void run(hideBreakWindow)}>Close for now</button>
  </main>
}
