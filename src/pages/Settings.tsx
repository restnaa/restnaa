import { Database } from 'lucide-react'
import { DeviceStatus } from '../components/DeviceStatus'
import type { AppInfoState } from '../lib/types'
import { NativePreferences } from '../components/NativePreferences'
import { ScheduleForm } from '../components/ScheduleForm'
import { ClearHistory } from '../components/ClearHistory'
import { PauseControls } from '../components/PauseControls'
import { PageHeader } from '../components/PageHeader'
import { IconBadge } from '../components/IconBadge'
import { useScheduler } from '../hooks/useScheduler'
export function Settings({ appInfo, onSetupComplete }: { appInfo: AppInfoState; onSetupComplete?: () => void }) {
  const view = useScheduler()
  const setup = view.status === 'ready' && !view.state.settings.onboardingCompleted
  return <>
    <PageHeader title="Settings" subtitle="A rhythm that fits your day." />
    {setup && <aside className="setup-note"><strong>Make Restnaa yours.</strong> Choose your reminders and finish setup. Start on login is optional. Closing keeps reminders in the tray; Quit stops the app.</aside>}
    {view.status === 'loading' && <p role="status">Loading your schedule…</p>}
    {view.status === 'error' && <p role="alert" className="error-message">{view.message}</p>}
    {view.status === 'ready' && <ScheduleForm initial={view.state.settings} onboarding={setup} onSaved={setup ? onSetupComplete : undefined} />}
    <div className="settings-bottom-grid">
      {view.status === 'ready' && <PauseControls settings={view.state.settings} paused={view.state.status === 'paused'} />}
      <NativePreferences />
      <section className="surface about-card" aria-labelledby="about-title"><div className="card-heading"><IconBadge icon={Database} /><div><h2 id="about-title">About &amp; data</h2><p>Local data. No account needed.</p></div></div><dl className="about-list"><div><dt>Local storage</dt><dd>{appInfo.status === 'loaded' && appInfo.info.storage.status === 'ready' ? 'Ready' : appInfo.status === 'loading' ? 'Checking…' : 'Unavailable'}</dd></div><div><dt>App version</dt><dd>{appInfo.status === 'loaded' ? appInfo.info.version : 'Unavailable'}</dd></div></dl><ClearHistory compact /><p className="field-help">A gentle companion, not a medical product.</p></section>
    </div>
    <details className="storage-details"><summary>Storage details</summary><DeviceStatus state={appInfo} /></details>
  </>
}
