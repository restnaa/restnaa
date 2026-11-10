import { vi } from 'vitest'
import { isTauri } from '@tauri-apps/api/core'
import { getAppInfo } from './api'
import { waitForDesktopReady } from './startup'
vi.mock('@tauri-apps/api/core',()=>({isTauri:vi.fn()}))
vi.mock('./api',()=>({getAppInfo:vi.fn()}))
afterEach(()=>{vi.useRealTimers()})
test('ordinary browser preview never fabricates native readiness or invokes Rust',async()=>{
  vi.mocked(isTauri).mockReturnValue(false)
  expect(await waitForDesktopReady()).toBe(false)
  expect(getAppInfo).not.toHaveBeenCalled()
})
test('retries only the read-only startup handshake until real metadata arrives',async()=>{
  vi.useFakeTimers();vi.mocked(isTauri).mockReturnValue(true)
  vi.mocked(getAppInfo).mockRejectedValueOnce('State not yet managed').mockResolvedValue({version:'0.1.1',storage:{status:'ready',schemaVersion:2}})
  const pending=waitForDesktopReady()
  await vi.advanceTimersByTimeAsync(100)
  expect(await pending).toBe(true)
  expect(getAppInfo).toHaveBeenCalledTimes(2)
  expect(vi.getTimerCount()).toBe(0)
})
test('permanent connection failure stops after five seconds without claiming success',async()=>{
  vi.useFakeTimers();vi.mocked(isTauri).mockReturnValue(true)
  vi.mocked(getAppInfo).mockRejectedValue('Connection unavailable')
  const pending=waitForDesktopReady()
  await vi.advanceTimersByTimeAsync(5000)
  expect(await pending).toBe(false)
  expect(vi.getTimerCount()).toBe(0)
})
test('a reported storage failure is valid metadata, not a reason to erase or retry storage',async()=>{
  vi.mocked(isTauri).mockReturnValue(true)
  vi.mocked(getAppInfo).mockResolvedValue({version:'0.1.1',storage:{status:'unavailable',message:'Existing data preserved'}})
  expect(await waitForDesktopReady()).toBe(true)
  expect(getAppInfo).toHaveBeenCalledTimes(1)
})

test('an IPC read that never settles cannot keep the startup screen indefinitely',async()=>{
  vi.useFakeTimers();vi.mocked(isTauri).mockReturnValue(true)
  vi.mocked(getAppInfo).mockImplementation(()=>new Promise(()=>{}))
  const pending=waitForDesktopReady()
  await vi.advanceTimersByTimeAsync(5000)
  expect(await pending).toBe(false)
  expect(vi.getTimerCount()).toBe(0)
})
