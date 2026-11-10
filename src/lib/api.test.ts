import { vi } from 'vitest'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { getAppInfo } from './api'
import * as api from './api'
import { listen } from '@tauri-apps/api/event'
import { schedulerState } from '../test/schedulerFixtures'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(), isTauri: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))

test('browser preview never pretends to have a database', async () => {
  vi.mocked(isTauri).mockReturnValue(false)
  await expect(getAppInfo()).rejects.toThrow('Desktop connection unavailable')
  expect(invoke).not.toHaveBeenCalled()
})

test('desktop uses the narrow read-only command and returns its data', async () => {
  vi.mocked(isTauri).mockReturnValue(true)
  const info = { version: '0.1.0', storage: { status: 'ready', schemaVersion: 1 } }
  vi.mocked(invoke).mockResolvedValue(info)
  expect(await getAppInfo()).toEqual(info)
  expect(invoke).toHaveBeenCalledWith('get_app_info')
})

test('scheduler actions send typed arguments to Rust without frontend timing', async () => {
  vi.mocked(isTauri).mockReturnValue(true)
  vi.mocked(invoke).mockResolvedValue(schedulerState())
  await api.updateSettings(schedulerState().settings)
  expect(invoke).toHaveBeenLastCalledWith('update_settings', { settings: schedulerState().settings })
  await api.pauseReminders({ kind: 'manual' })
  expect(invoke).toHaveBeenLastCalledWith('pause_reminders', { request: { kind: 'manual' } })
  await api.resumeReminders()
  expect(invoke).toHaveBeenLastCalledWith('resume_reminders')
  await api.startBreak('eye')
  expect(invoke).toHaveBeenLastCalledWith('start_break', { category: 'eye' })
  for (const [method, command] of [[api.completeBreak, 'complete_break'], [api.skipBreak, 'skip_break'], [api.snoozeBreak, 'snooze_break']] as const) {
    await method('reminder-id')
    expect(invoke).toHaveBeenLastCalledWith(command, { reminderId: 'reminder-id' })
  }
})

test('typed scheduler subscriptions forward payloads and expose cleanup', async () => {
  vi.mocked(isTauri).mockReturnValue(true)
  const stop = vi.fn()
  vi.mocked(listen).mockResolvedValue(stop)
  const callback = vi.fn()
  expect(await api.subscribeScheduler('reminder-due', callback)).toBe(stop)
  const receive = vi.mocked(listen).mock.calls[0][1]
  const payload = { id: 'id', category: 'eye', triggeredAtUtc: 'now', expiresAtUtc: 'later', startedAtUtc: null }
  receive({ event: 'reminder-due', id: 1, payload })
  expect(callback).toHaveBeenCalledWith(payload)
})

test('native controls use dedicated commands without exposing plugin mutations', async () => {
  vi.mocked(isTauri).mockReturnValue(true)
  await api.setAutostartEnabled(true)
  expect(invoke).toHaveBeenLastCalledWith('set_autostart_enabled', { enabled: true })
  await api.setNotificationsEnabled(false)
  expect(invoke).toHaveBeenLastCalledWith('set_notifications_enabled', { enabled: false })
  for (const [method, command] of [[api.getNativeStatus, 'get_native_status'], [api.testNotification, 'test_notification'], [api.openCurrentBreak, 'open_current_break'], [api.hideBreakWindow, 'hide_break_window'], [api.quitApp, 'quit_app']] as const) {
    await method()
    expect(invoke).toHaveBeenLastCalledWith(command)
  }
})

test('history has typed periods and explicit deletion confirmation',async()=>{
  vi.mocked(isTauri).mockReturnValue(true)
  await api.getHistory('sevenDays')
  expect(invoke).toHaveBeenLastCalledWith('get_history',{range:'sevenDays'})
  await api.clearHistory(true)
  expect(invoke).toHaveBeenLastCalledWith('clear_history',{confirmed:true})
})
