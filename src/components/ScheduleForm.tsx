import { MovementIcon } from './MovementIcon'
import { useState, type FormEvent } from 'react'
import { getSettings, updateSettings } from '../lib/api'
import { CalendarDays, Eye } from 'lucide-react'
import { IconBadge } from './IconBadge'
import type { SchedulerSettings } from '../lib/types'

const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
export function ScheduleForm({ initial, onboarding = false, onSaved }: { initial: SchedulerSettings; onboarding?: boolean; onSaved?: () => void }) {
  const [draft, setDraft] = useState(initial)
  const [busy, setBusy] = useState(false)
  const [message, setMessage] = useState('')
  const [error, setError] = useState('')
  const change = (patch: Partial<SchedulerSettings>) => { setDraft(current => ({ ...current, ...patch })); setMessage(''); setError('') }
  const submit = async (event: FormEvent) => {
    event.preventDefault()
    if (busy) return
    setError(''); setMessage('')
    if (!draft.workDays.length) { setError('Choose at least one working day.'); return }
    if (draft.workEndLocal <= draft.workStartLocal) { setError('Working hours must end after they start. Overnight schedules are not supported.'); return }
    setBusy(true)
    try {
      const latest = await getSettings()
      await updateSettings({ ...latest, eyeEnabled: draft.eyeEnabled, movementEnabled: draft.movementEnabled,
        eyeIntervalMinutes: draft.eyeIntervalMinutes, movementIntervalMinutes: draft.movementIntervalMinutes,
        showBreakWindow: draft.showBreakWindow, workDays: draft.workDays, workStartLocal: draft.workStartLocal, workEndLocal: draft.workEndLocal, quietMode: draft.quietMode,
        onboardingCompleted: onboarding || latest.onboardingCompleted })
      setMessage('Saved on this device. Your schedule is now up to date.')
      onSaved?.()
    } catch (reason) { setError(typeof reason === 'string' ? reason : 'Settings could not be saved. Your previous schedule remains in place.') }
    finally { setBusy(false) }
  }
  return <form className="schedule-form" onSubmit={event => void submit(event)} aria-label={onboarding ? 'Set up reminders' : 'Reminder settings'} aria-busy={busy}>
    <div className="settings-top-grid">
      <fieldset className="surface reminder-settings" disabled={busy}>
        <legend className="sr-only">Reminders</legend>
        <div className="card-heading"><IconBadge icon={Eye} /><div><h2>Reminders</h2><p>Gentle reminders to rest and move.</p></div></div>
        <div className="reminder-setting-row"><IconBadge icon={Eye} small /><label className="preference-toggle"><span>Eye reminders</span><input className="switch" type="checkbox" checked={draft.eyeEnabled} onChange={event => change({ eyeEnabled: event.target.checked })} /></label><label className="interval-field"><span aria-hidden="true">Every</span><span className="sr-only">Eye interval (minutes)</span><input aria-label="Eye interval (minutes)" type="number" required min={10} max={120} step={1} value={Number.isNaN(draft.eyeIntervalMinutes) ? '' : draft.eyeIntervalMinutes} onChange={event => change({ eyeIntervalMinutes: event.target.valueAsNumber })} /><span aria-hidden="true">min</span></label></div>
        <div className="reminder-setting-row"><IconBadge icon={MovementIcon} small violet /><label className="preference-toggle"><span>Movement reminders</span><input className="switch" type="checkbox" checked={draft.movementEnabled} onChange={event => change({ movementEnabled: event.target.checked })} /></label><label className="interval-field"><span aria-hidden="true">Every</span><span className="sr-only">Movement interval (minutes)</span><input aria-label="Movement interval (minutes)" type="number" required min={15} max={180} step={1} value={Number.isNaN(draft.movementIntervalMinutes) ? '' : draft.movementIntervalMinutes} onChange={event => change({ movementIntervalMinutes: event.target.valueAsNumber })} /><span aria-hidden="true">min</span></label></div>
        <label className="preference-toggle automatic-window-toggle"><input className="switch" type="checkbox" checked={draft.showBreakWindow} onChange={event => change({ showBreakWindow: event.target.checked })} /><span>Show break window when a reminder is due</span></label><p className="field-help">Requires notifications enabled. Opens without taking keyboard focus. Windows DND is best effort; use Restnaa Quiet mode or Pause for guaranteed quiet.</p>
        <p className="field-help interval-help">Eye: 10–120 min · Movement: 15–180 min</p>
      </fieldset>
      <fieldset className="surface work-settings" disabled={busy}><legend className="sr-only">Working schedule</legend>
        <div className="card-heading"><IconBadge icon={CalendarDays} /><div><h2>Working hours</h2><p>Reminders stay quiet outside these hours.</p></div></div>
        <div className="day-options">{days.map((day, index) => <label key={day} className={draft.workDays.includes(index) ? 'selected' : ''}><input type="checkbox" aria-label={day} checked={draft.workDays.includes(index)} onChange={event => change({ workDays: event.target.checked ? [...draft.workDays, index].sort() : draft.workDays.filter(value => value !== index) })} /><span aria-hidden="true">{day.slice(0,3)}</span></label>)}</div>
        <div className="form-grid"><label className="field">Start time<input aria-label="Work starts" type="time" required step={60} value={draft.workStartLocal.slice(0,5)} onChange={event => change({ workStartLocal: `${event.target.value}:00` })} /></label><label className="field">End time<input aria-label="Work ends" type="time" required step={60} value={draft.workEndLocal.slice(0,5)} onChange={event => change({ workEndLocal: `${event.target.value}:00` })} /></label></div>
        <label className="quiet-toggle"><input type="checkbox" checked={draft.quietMode} onChange={event => change({ quietMode: event.target.checked })} /><span>Quiet mode — keep all reminders quiet</span></label>
      </fieldset>
    </div>
    <div className="schedule-save"><p className="field-help">Uses device local time. Saving starts fresh intervals and closes any open reminder as unanswered.</p><button type="submit" className="primary-button" disabled={busy}>{busy ? 'Saving…' : onboarding ? 'Finish setup' : 'Save settings'}</button></div>
    {error && <p role="alert" className="error-message">{error}</p>}
    {message && <p role="status" className="save-message">{message}</p>}
  </form>
}
