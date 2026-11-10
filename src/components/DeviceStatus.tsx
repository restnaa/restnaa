import { Bell, Database, Laptop } from 'lucide-react'
import type { AppInfoState } from '../lib/types'
import { useNativeStatus } from '../hooks/useNativeStatus'
import { IconBadge } from './IconBadge'
export function DeviceStatus({ state }: { state: AppInfoState }) {
  const { status } = useNativeStatus()
  const storage = state.status === 'loaded' ? state.info.storage : null
  const error = state.status === 'error' ? state.message : storage?.status === 'unavailable' ? storage.message : null
  const notifications = !status ? 'Unavailable' : !status.notificationEnabled ? 'Off in app' : status.windowsPermission === 'unknown' ? 'Permission unknown' : status.windowsPermission !== 'allowed' ? 'Blocked by Windows' : status.attention !== 'accepting' && status.attention !== 'unknown' ? 'May be suppressed' : 'Enabled in app'
  return <section className="surface device-status" aria-labelledby="device-title">
    <div className="card-heading"><IconBadge icon={Laptop} small /><h2 id="device-title">On this device</h2></div>
    <dl className="device-list"><div><dt><Database aria-hidden="true" />Local storage</dt><dd><span role="status" className={storage?.status === 'ready' ? 'status-good' : ''}>{state.status === 'loading' ? 'Checking…' : storage?.status === 'ready' ? `Ready · schema ${storage.schemaVersion}` : 'Unavailable'}</span></dd></div><div><dt><Bell aria-hidden="true" />Notifications</dt><dd>{notifications}</dd></div></dl>
    <p className="field-help">Local data. No account needed.</p>
    {error && <p role="alert" className="error-message">{error}</p>}
  </section>
}
