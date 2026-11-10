import { useState } from 'react'
import { Pause, Play } from 'lucide-react'
import { IconBadge } from './IconBadge'
import { pauseReminders, resumeReminders } from '../lib/api'
import type { PauseRequest, SchedulerSettings } from '../lib/types'

const options: { label: string; request: PauseRequest }[] = [
  { label: '30 minutes', request: { kind: 'minutes', minutes: 30 } }, { label: '1 hour', request: { kind: 'minutes', minutes: 60 } },
  { label: '2 hours', request: { kind: 'minutes', minutes: 120 } }, { label: 'Until tomorrow', request: { kind: 'untilTomorrow' } }, { label: 'Until I resume', request: { kind: 'manual' } },
]
export function PauseControls({ settings, paused }: { settings: SchedulerSettings; paused: boolean }) {
  const [choice, setChoice] = useState('0')
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState('')
  const run = async (resume: boolean) => {
    if (busy) return
    setBusy(true); setError('')
    try { if (resume) await resumeReminders(); else await pauseReminders(options[Number(choice)].request) }
    catch (reason) { setError(typeof reason === 'string' ? reason : 'Could not change pause state. Please try again.') }
    finally { setBusy(false) }
  }
  return <section className="surface pause-card" aria-labelledby="pause-title">
    <div className="card-heading"><IconBadge icon={Pause} small /><h2 id="pause-title">Focus &amp; pause</h2></div>
    <p className="pause-description">{paused ? settings.pausedIndefinitely ? 'Paused until you choose Resume.' : `Paused until ${new Date(settings.pausedUntilUtc!).toLocaleString()}.` : 'Pause reminders for a while.'}</p>
    <label className="field"><span className="sr-only">Pause for</span><select value={choice} disabled={busy} onChange={event => setChoice(event.target.value)}>{options.map((option, index) => <option key={option.label} value={index}>{option.label}</option>)}</select></label>
    <button type="button" className="primary-button full-width" disabled={busy} onClick={() => void run(false)}><Pause fill="currentColor" aria-hidden="true" />{paused ? 'Update pause' : 'Pause reminders'}</button>
    <button type="button" className="secondary-button full-width" disabled={busy || !paused} onClick={() => void run(true)}><Play fill="currentColor" aria-hidden="true" />Resume reminders</button>
    {choice === '3' && <p className="field-help">Until tomorrow ends at local midnight.</p>}
    {error && <p role="alert" className="error-message">{error}</p>}
  </section>
}
