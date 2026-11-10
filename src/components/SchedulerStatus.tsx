import { MovementIcon } from './MovementIcon'
import { useScheduler } from '../hooks/useScheduler'
import { openCurrentBreak } from '../lib/api'
import { useState } from 'react'
import { CalendarDays, Eye } from 'lucide-react'
import { IconBadge } from './IconBadge'
import { useDisplayTime } from '../hooks/useDisplayTime'
import { deadlineLabel, showDeadline } from '../lib/deadlines'
import type { DashboardState } from '../lib/types'
const schedulerLabels = { active: 'Active', paused: 'Paused', outsideWorkHours: 'Outside work hours', quiet: 'Quiet mode', disabled: 'Reminders disabled' }
function ScheduleContent({ state }: { state: DashboardState }) {
  const now = useDisplayTime()
  const [error, setError] = useState<string | null>(null)
  return <>
    <p role="status" className="schedule-state">{schedulerLabels[state.status]}</p>
    <dl className="schedule-list">{state.reminders.map(reminder => <div key={reminder.category}>
      <dt><IconBadge icon={reminder.category === 'eye' ? Eye : MovementIcon} small violet={reminder.category === 'movement'} />Next {reminder.category} break</dt>
      <dd>{showDeadline(state, reminder) && <time dateTime={reminder.nextDueAtUtc} title={new Date(reminder.nextDueAtUtc).toLocaleString()}>{new Date(reminder.nextDueAtUtc).toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' })}</time>}<span>{deadlineLabel(state, reminder, now)}</span></dd>
    </div>)}</dl>
    {state.openReminder && <button type="button" className="secondary-button full-width" onClick={() => { setError(null); void openCurrentBreak().catch(() => setError('Could not open this break. It may have expired; check the tray.')) }}>Open current break</button>}
    {error && <p role="alert" className="error-message">{error}</p>}
  </>
}
export function SchedulerStatus({ state }: { state?: DashboardState }) {
  return <section className="surface scheduler-status" aria-labelledby="scheduler-title"><div className="card-heading"><IconBadge icon={CalendarDays} small /><h2 id="scheduler-title">Your schedule</h2></div>{state ? <ScheduleContent state={state} /> : <ConnectedSchedule />}</section>
}
function ConnectedSchedule() {
  const view = useScheduler()
  return view.status === 'ready' ? <ScheduleContent state={view.state} /> : view.status === 'loading' ? <p role="status">Connecting to the Rust scheduler…</p> : <p role="alert" className="error-message">{view.message}</p>
}
