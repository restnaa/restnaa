import { act, renderHook, waitFor } from '@testing-library/react'
import { vi } from 'vitest'
import { useScheduler } from './useScheduler'
import { getDashboardState, subscribeScheduler } from '../lib/api'
import type { DashboardState, SchedulerStateEvent } from '../lib/types'
import { schedulerState } from '../test/schedulerFixtures'

vi.mock('../lib/api', () => ({ getDashboardState: vi.fn(), subscribeScheduler: vi.fn() }))

function deferred<T>() {
  let resolve!: (value: T) => void
  const promise = new Promise<T>(yes => { resolve = yes })
  return { promise, resolve }
}

test('subscribes before initial invoke and rejects a stale snapshot response', async () => {
  const initial = deferred<DashboardState>()
  const stop = vi.fn()
  let notify!: (event: SchedulerStateEvent) => void
  vi.mocked(subscribeScheduler).mockImplementation(async (_name, callback) => {
    notify = callback as typeof notify
    return stop
  })
  vi.mocked(getDashboardState).mockReturnValue(initial.promise)
  const { result, unmount } = renderHook(useScheduler)
  await waitFor(() => expect(getDashboardState).toHaveBeenCalledOnce())
  expect(vi.mocked(subscribeScheduler).mock.invocationCallOrder[0]).toBeLessThan(vi.mocked(getDashboardState).mock.invocationCallOrder[0])
  act(() => notify({ kind: 'state', state: schedulerState(3) }))
  await act(async () => initial.resolve(schedulerState(1)))
  expect(result.current).toEqual({ status: 'ready', state: schedulerState(3) })
  unmount()
  expect(stop).toHaveBeenCalledOnce()
})

test('late subscription registration is cleaned up after unmount without invoking', async () => {
  const subscription = deferred<() => void>()
  vi.mocked(subscribeScheduler).mockReturnValue(subscription.promise)
  const { unmount } = renderHook(useScheduler)
  unmount()
  const stop = vi.fn()
  await act(async () => subscription.resolve(stop))
  expect(stop).toHaveBeenCalledOnce()
  expect(getDashboardState).not.toHaveBeenCalled()
})

test('persistence failure stays visible even if an older initial response arrives', async () => {
  const initial = deferred<DashboardState>()
  let notify!: (event: SchedulerStateEvent) => void
  vi.mocked(subscribeScheduler).mockImplementation(async (_name, callback) => { notify = callback as typeof notify; return () => {} })
  vi.mocked(getDashboardState).mockReturnValue(initial.promise)
  const { result } = renderHook(useScheduler)
  await waitFor(() => expect(getDashboardState).toHaveBeenCalledOnce())
  act(() => notify({ kind: 'error', message: 'Scheduler stopped: local save failed.' }))
  await act(async () => initial.resolve(schedulerState(1)))
  expect(result.current).toEqual({ status: 'error', message: 'Scheduler stopped: local save failed.' })
})

test('subscription and snapshot failures are visible', async () => {
  vi.mocked(subscribeScheduler).mockRejectedValueOnce(new Error('permission denied'))
  const first = renderHook(useScheduler)
  await waitFor(() => expect(first.result.current.status).toBe('error'))
  first.unmount()
  vi.mocked(subscribeScheduler).mockResolvedValueOnce(vi.fn())
  vi.mocked(getDashboardState).mockRejectedValueOnce(new Error('storage unavailable'))
  const second = renderHook(useScheduler)
  await waitFor(() => expect(second.result.current.status).toBe('error'))
})
