import { MovementIcon } from '../components/MovementIcon'
import { useState } from 'react'
import { ChevronDown, Eye, RefreshCw } from 'lucide-react'
import { useHistory } from '../hooks/useHistory'
import { HistorySummary } from '../components/HistorySummary'
import { IconBadge } from '../components/IconBadge'
import { PageHeader } from '../components/PageHeader'
import { outcomeLabels } from '../lib/labels'
import type { HistoryRange, LocalHistory } from '../lib/types'
const localDateKey = (date: Date) => `${date.getFullYear()}-${String(date.getMonth()+1).padStart(2,'0')}-${String(date.getDate()).padStart(2,'0')}`
export function ActivityGroups({ history }: { history: LocalHistory }) {
  const groups = new Map<string, LocalHistory['recent']>()
  for (const event of history.recent) {
    const key = localDateKey(new Date(event.actedAtUtc))
    const events = groups.get(key) ?? []
    events.push(event); groups.set(key, events)
  }
  const today = localDateKey(new Date(history.asOfUtc))
  return <div className="activity-groups">{[...history.days].sort((a,b) => b.date.localeCompare(a.date)).map((day, index) => <details className="activity-group" key={day.date} open={index === 0}>
    <summary><span className="day-label">{day.date === today ? 'Today' : new Date(`${day.date}T12:00:00`).toLocaleDateString([], {weekday:'short'})}</span><time dateTime={day.date}>{new Date(`${day.date}T12:00:00`).toLocaleDateString([], {month:'short',day:'numeric',year:'numeric'})}</time><span className="day-counts">{day.categories.map(item => <span key={item.category}><span className="sr-only">Completed {item.category} breaks: </span>{item.category === 'eye' ? <Eye aria-hidden="true" /> : <MovementIcon aria-hidden="true" />}{item.counts.completed}</span>)}</span><ChevronDown className="disclosure-icon" aria-hidden="true" /></summary>
    <ol className="event-list">{(groups.get(day.date) ?? []).map(event => <li key={event.id}><time dateTime={event.actedAtUtc}>{new Date(event.actedAtUtc).toLocaleTimeString([], {hour:'numeric',minute:'2-digit'})}</time><IconBadge icon={event.category === 'eye' ? Eye : MovementIcon} small violet={event.category === 'movement'} /><strong>{event.category === 'eye' ? 'Eye break' : 'Movement break'}</strong><span className={`outcome-label ${event.outcome}`}>{outcomeLabels[event.outcome]}</span></li>)}</ol>
    {!(groups.get(day.date)?.length) && <p className="day-empty">{day.categories.some(item => Object.values(item.counts).some(count => count > 0)) ? 'Earlier outcomes are outside the latest 100 shown.' : 'No recorded outcomes on this day.'}</p>}
    <details className="day-totals"><summary>Day totals by outcome</summary><HistorySummary categories={day.categories} /></details>
  </details>)}</div>
}
export function History() {
  const [range, setRange] = useState<HistoryRange>('sevenDays')
  const { view, refresh } = useHistory(range)
  return <>
    <PageHeader title="History" subtitle="A simple look back." />
    <div className="history-top-grid">
      <section className="surface history-filter"><label className="field">Time range<select aria-label="Show history" value={range} onChange={event => setRange(event.target.value as HistoryRange)}><option value="today">Today</option><option value="sevenDays">Last 7 days</option></select></label><button type="button" className="secondary-button" onClick={refresh}><RefreshCw aria-hidden="true" />Refresh history</button></section>
      {view.status === 'ready' && view.history.categories.map(item => <section className="surface history-stat" key={item.category}><IconBadge icon={item.category === 'eye' ? Eye : MovementIcon} violet={item.category === 'movement'} /><div><h2>{item.category === 'eye' ? 'Eye breaks' : 'Movement breaks'}</h2><strong>{item.counts.completed}</strong><p>completed {range === 'today' ? 'today' : 'in last 7 days'}</p></div></section>)}
    </div>
    {view.status === 'loading' && <p role="status">Reading local history…</p>}
    {view.status === 'error' && <p role="alert" className="error-message">{view.message}</p>}
    {view.status === 'ready' && <section className="surface activity-card" aria-labelledby="activity-title"><h2 id="activity-title">Recent activity</h2><p className="history-period">{view.history.fromDate} – {view.history.throughDate}</p>
      {view.history.totalEvents === 0 ? <p className="empty-history">No recorded outcomes in this period. Your next pause is yours to choose.</p> : <>
        <ActivityGroups history={view.history} />
        <p className="field-help">{view.history.recentTruncated ? `Showing the latest 100 of ${view.history.totalEvents} outcomes. Totals include the whole period.` : `${view.history.totalEvents} recorded outcomes in this period.`}</p>
      </>}
      <details className="history-explanation"><summary>All outcomes &amp; how they count</summary><HistorySummary categories={view.history.categories} /><p className="field-help">Completed means you chose Done, not proof of an exercise. Snoozed delays a reminder five minutes. Unanswered means it expired or was dismissed by a schedule or pause change. Dates use your current device timezone.</p></details>
    </section>}
  </>
}
