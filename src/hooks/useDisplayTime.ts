import { useEffect, useState } from 'react'
// A low-frequency, visible-window display refresh, never a reminder timer.
export function useDisplayTime() {
  const [now, setNow] = useState(() => Date.now())
  useEffect(() => {
    let timeout: ReturnType<typeof setTimeout> | undefined
    const refresh = () => {
      clearTimeout(timeout)
      if (document.visibilityState === 'hidden') return
      setNow(Date.now())
      timeout = setTimeout(refresh, 30000)
    }
    refresh()
    document.addEventListener('visibilitychange', refresh)
    window.addEventListener('focus', refresh)
    return () => { clearTimeout(timeout); document.removeEventListener('visibilitychange', refresh); window.removeEventListener('focus', refresh) }
  }, [])
  return now
}
