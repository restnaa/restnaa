import { act, renderHook } from '@testing-library/react'
import { vi } from 'vitest'
import { useDisplayTime } from './useDisplayTime'
afterEach(() => { vi.useRealTimers(); vi.restoreAllMocks() })
test('refreshes display time only while visible and cleans up when unmounted', () => {
  vi.useFakeTimers(); vi.setSystemTime(new Date('2026-10-21T09:00:00Z'))
  let visibility: DocumentVisibilityState = 'visible'
  vi.spyOn(document,'visibilityState','get').mockImplementation(() => visibility)
  const {result,unmount} = renderHook(useDisplayTime)
  const first=result.current
  act(()=>{vi.advanceTimersByTime(30000)})
  expect(result.current).toBe(first+30000)
  visibility='hidden'
  act(()=>{document.dispatchEvent(new Event('visibilitychange'))})
  expect(vi.getTimerCount()).toBe(0)
  act(()=>{vi.advanceTimersByTime(120000)})
  expect(result.current).toBe(first+30000)
  visibility='visible'
  act(()=>{document.dispatchEvent(new Event('visibilitychange'))})
  expect(result.current).toBe(first+150000)
  unmount();expect(vi.getTimerCount()).toBe(0)
})
