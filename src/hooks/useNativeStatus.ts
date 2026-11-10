import { useCallback, useEffect, useRef, useState } from 'react'
import { getNativeStatus, subscribeScheduler } from '../lib/api'
import type { NativeStatus } from '../lib/types'

export function useNativeStatus() {
  const [status, setStatus] = useState<NativeStatus | null>(null)
  const [error, setError] = useState<string | null>(null)
  const active = useRef(false)
  const sequence = useRef(0)
  const stopLoading = useCallback(() => { active.current = false; sequence.current++ }, [])
  const refresh = useCallback(async () => {
    const request = ++sequence.current
    try {
      const value = await getNativeStatus()
      if (active.current && request === sequence.current) { setStatus(value); setError(null) }
    } catch {
      if (active.current && request === sequence.current) setError('Native controls are unavailable. Open the desktop app and check local storage.')
    }
  }, [])
  useEffect(() => {
    active.current = true
    let current = true
    let stop: (() => void) | undefined
    subscribeScheduler('native-status-changed', () => { if (current) void refresh() }).then(unlisten => {
      if (!current) { unlisten(); return }
      stop = unlisten
      void refresh()
    }).catch(() => { if (current) setError('Native controls are unavailable. Open the desktop app and check local storage.') })
    return () => { current = false; stopLoading(); stop?.() }
  }, [refresh, stopLoading])
  return { status, error, refresh }
}
