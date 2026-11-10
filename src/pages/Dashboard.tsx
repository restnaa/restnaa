import { MovementIcon } from '../components/MovementIcon'
import { useState } from 'react'
import { ChartNoAxesColumnIncreasing, Eye, Play, ChevronRight, RefreshCw } from 'lucide-react'
import type { AppInfoState, Category, Page } from '../lib/types'
import { DeviceStatus } from '../components/DeviceStatus'
import { SchedulerStatus } from '../components/SchedulerStatus'
import { PauseControls } from '../components/PauseControls'
import { HistorySummary } from '../components/HistorySummary'
import { IconBadge } from '../components/IconBadge'
import { PageHeader } from '../components/PageHeader'
import { useScheduler } from '../hooks/useScheduler'
import { useHistory } from '../hooks/useHistory'
import { useDisplayTime } from '../hooks/useDisplayTime'
import { deadlineLabel } from '../lib/deadlines'
import { startBreak, openCurrentBreak } from '../lib/api'
export function Dashboard({ appInfo, navigate }: { appInfo: AppInfoState; navigate: (page: Page) => void }) {
  const scheduler = useScheduler()
  const { view: history, refresh } = useHistory('today')
  const now = useDisplayTime()
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState('')
  const takeBreak = async (category: Category) => {
    if (busy) return
    setBusy(true); setError('')
    try { await startBreak(category); await openCurrentBreak() }
    catch (reason) { setError(typeof reason === 'string' ? reason : 'Could not open this break. Check your current schedule or the tray.') }
    finally { setBusy(false) }
  }
  return <>
    <PageHeader title="A little space to pause." subtitle="Rest your eyes. Move your body. Keep your focus." />
    {scheduler.status === 'loading' && <p role="status">Connecting to the Rust scheduler…</p>}
    {scheduler.status === 'error' && <p role="alert" className="error-message">{scheduler.message}</p>}
    {error && <p role="alert" className="error-message">{error}</p>}
    <div className="dashboard-grid">
      {scheduler.status === 'ready' && <>
        {scheduler.state.reminders.map(reminder => <section className={`surface break-card ${reminder.category}`} key={reminder.category}>
          <div className="break-card-top"><IconBadge icon={reminder.category === 'eye' ? Eye : MovementIcon} violet={reminder.category === 'movement'} /><span className="status-pill">{deadlineLabel(scheduler.state, reminder, now)}</span></div>
          <h2>{reminder.category === 'eye' ? 'Eye break' : 'Movement break'}</h2><p>{reminder.category === 'eye' ? 'Look at something far away for 20 seconds.' : 'Stand, stretch gently, or take a short walk.'}</p>
          <button type="button" className={`primary-button${reminder.category === 'movement' ? ' violet-button' : ''}`} disabled={busy || scheduler.state.status !== 'active' || !!scheduler.state.openReminder || !(reminder.category === 'eye' ? scheduler.state.settings.eyeEnabled : scheduler.state.settings.movementEnabled)} onClick={() => void takeBreak(reminder.category)}><Play fill="currentColor" aria-hidden="true" />Take {reminder.category} break</button>
        </section>)}
        <SchedulerStatus state={scheduler.state} />
        <PauseControls settings={scheduler.state.settings} paused={scheduler.state.status === 'paused'} />
      </>}
      <section className="surface today-card" aria-labelledby="today-title"><div className="card-heading"><IconBadge icon={ChartNoAxesColumnIncreasing} small /><h2 id="today-title">Today</h2><button type="button" className="icon-button" aria-label="View seven-day history" onClick={() => navigate('history')}><ChevronRight aria-hidden="true" /></button></div>
        {history.status === 'loading' && <p role="status">Reading local history…</p>}
        {history.status === 'error' && <p role="alert" className="error-message">{history.message}</p>}
        {history.status === 'ready' && <HistorySummary categories={history.history.categories} />}
        <button type="button" className="text-button refresh-today" onClick={refresh}><RefreshCw size={14} aria-hidden="true" />Refresh today</button>
      </section>
      <DeviceStatus state={appInfo} />
    </div>
  </>
}
