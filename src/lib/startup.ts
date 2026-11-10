import { isTauri } from '@tauri-apps/api/core'
import { getAppInfo } from './api'

/** Windows can load the webview before setup has registered Rust state.
 * Only metadata reads are retried; the scheduler and user actions never are.
 * False is a bounded failure, not permission to fabricate a ready backend.
 */
export async function waitForDesktopReady(): Promise<boolean> {
  const nativeHost = window.location.protocol === 'tauri:' || window.location.hostname === 'tauri.localhost'
  if (!isTauri() && !nativeHost) return false
  let expired = false
  let deadline: ReturnType<typeof setTimeout> | undefined
  const timeout = new Promise<boolean>(resolve => {
    deadline = setTimeout(() => { expired = true; resolve(false) }, 5000)
  })
  const read = async () => {
    while (!expired) {
      try { await getAppInfo(); return !expired }
      catch { /* IPC or Rust setup may still be initializing. */ }
      if (expired) break
      await new Promise<void>(resolve => setTimeout(resolve, 100))
    }
    return false
  }
  try { return await Promise.race([read(), timeout]) }
  finally { clearTimeout(deadline) }
}
