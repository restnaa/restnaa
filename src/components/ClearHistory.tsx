import { useRef, useState } from 'react'
import { clearHistory } from '../lib/api'
export function ClearHistory({ compact = false }: { compact?: boolean }) {
  const dialog = useRef<HTMLDialogElement>(null)
  const cancel = useRef<HTMLButtonElement>(null)
  const trigger = useRef<HTMLButtonElement>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState('')
  const [message, setMessage] = useState('')
  const close = () => { dialog.current?.close(); trigger.current?.focus() }
  const erase = async () => {
    if (busy) return
    setBusy(true); setError(''); setMessage('')
    try { await clearHistory(true); close(); setMessage('Local break history cleared. Your settings and current schedule are unchanged.') }
    catch (reason) { setError(typeof reason === 'string' ? reason : 'History could not be cleared. Please try again.') }
    finally { setBusy(false) }
  }
  return <section className={compact ? 'clear-history-compact' : 'surface content-panel'}>{!compact && <h2>Your local data</h2>}<p className="field-help">Erase recorded outcomes. Preferences and current reminders stay in place.</p>
    <button ref={trigger} type="button" className="secondary-button" onClick={() => { setError(''); setMessage(''); dialog.current?.showModal(); cancel.current?.focus() }}>Clear history…</button>
    {message && <p role="status" className="save-message">{message}</p>}
    <dialog ref={dialog} aria-labelledby="clear-title" aria-describedby="clear-description" onCancel={event => { if (busy) event.preventDefault() }} onClose={() => trigger.current?.focus()} onKeyDown={event => {
      if (event.key !== 'Tab') return
      const controls = dialog.current?.querySelectorAll<HTMLButtonElement>('button:not(:disabled)')
      if (!controls?.length) { event.preventDefault(); return }
      const first = controls[0], last = controls[controls.length - 1]
      if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus() }
      else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus() }
    }}>
      <h2 id="clear-title">Clear all break history?</h2><p id="clear-description">This permanently removes every completed, skipped, snoozed and unanswered event. This cannot be undone.</p>
      {error && <p role="alert" className="error-message">{error}</p>}
      <div className="native-actions"><button ref={cancel} type="button" className="primary-button" disabled={busy} onClick={close}>Keep history</button><button type="button" className="secondary-button danger-button" disabled={busy} onClick={() => void erase()}>{busy ? 'Clearing…' : 'Permanently clear history'}</button></div>
    </dialog>
  </section>
}
