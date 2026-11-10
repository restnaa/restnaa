import { useState } from 'react'
import { Bell, Play, RefreshCw, Power } from 'lucide-react'
import { IconBadge } from './IconBadge'
import { useNativeStatus } from '../hooks/useNativeStatus'
import { setAutostartEnabled, setNotificationsEnabled, testNotification, quitApp } from '../lib/api'

export function NativePreferences() {
  const { status, error, refresh } = useNativeStatus()
  const [busy, setBusy] = useState(false)
  const [actionError, setActionError] = useState<string | null>(null)
  const run = async (action: () => Promise<unknown>) => {
    if (busy) return
    setBusy(true); setActionError(null)
    try { await action(); await refresh() }
    catch (reason) { setActionError(typeof reason === 'string' ? reason : 'The native action could not be completed. Please try again.') }
    finally { setBusy(false) }
  }
  return <section className="surface native-card" aria-labelledby="native-title">
    <div className="card-heading"><IconBadge icon={Bell} small violet /><h2 id="native-title">Notifications &amp; system</h2></div>
    {error && <p role="alert" className="error-message">{error}</p>}
    {actionError && <p role="alert" className="error-message">{actionError}</p>}
    {!status && !error && <p role="status">Checking Windows controls…</p>}
    {status && <>
      <label className="preference-toggle"><input className="switch" type="checkbox" checked={status.notificationEnabled} disabled={busy} onChange={event => { const enabled = event.target.checked; void run(() => setNotificationsEnabled(enabled)) }} /><span>Enable notifications</span></label>
      <label className="preference-toggle"><input className="switch" type="checkbox" checked={status.autostartRegistered ?? false} disabled={busy || status.autostartRegistered === null} onChange={event => { const enabled = event.target.checked; void run(() => setAutostartEnabled(enabled)) }} /><span>Start on login</span></label>
      <p className="field-help">Start on login is off by default.</p><details className="native-diagnostics"><summary>Windows diagnostics</summary>
      <dl>
        <div><dt>Tray</dt><dd>{status.trayAvailable ? 'Available' : 'Unavailable — closing the dashboard exits the app'}</dd></div>
        <div><dt>Windows notifications</dt><dd>{status.windowsPermission === 'allowed' ? 'Allowed by app notification settings' : status.windowsPermission === 'unknown' ? 'Could not determine permission' : 'Disabled by Windows or policy'}</dd></div>
        <div><dt>Attention hint</dt><dd>{status.attention === 'accepting' ? 'No suppression reported — DND may still be on' : status.attention === 'unknown' ? 'Unknown' : 'Windows may suppress notifications'}</dd></div>
      </dl>
      <p className="native-help">Do not disturb/Focus Assist may hide banners even when notifications are allowed. Use Open current break in the tray if needed. Clicking an on-screen banner may open its break; Action Center clicks are not guaranteed.</p>
      </details>
      {status.autostartRegistered === null && <p role="alert">Could not read Windows startup registration. No startup change was made.</p>}
      {status.autostartRegistered !== null && status.autostartEnabled !== status.autostartRegistered && <p role="alert">Windows startup registration differs from your saved preference. Choose the Start on login setting you want.</p>}
      {status.warning && <p role="alert" className="error-message">{status.warning}</p>}

      {status.lastAttempt && <p role="status" className="native-help">{status.lastAttempt.message}</p>}
    </>}
    <div className="native-actions">
      {status && <button type="button" className="secondary-button" disabled={busy || !status.notificationEnabled} onClick={() => void run(testNotification)}><Play aria-hidden="true" />Send test reminder</button>}
      <button type="button" className="secondary-button" disabled={busy} onClick={() => void run(refresh)}><RefreshCw aria-hidden="true" />Refresh diagnostics</button>
      <button type="button" className="secondary-button" disabled={busy} onClick={() => void run(quitApp)}><Power aria-hidden="true" />Quit Restnaa completely</button>
    </div>
  </section>
}
