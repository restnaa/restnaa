import { invoke, isTauri } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { AppInfo, Category, DashboardState, PauseRequest, SchedulerEvents, SchedulerSettings, NativeStatus, NotificationAttempt } from './types'
import type { HistoryRange, LocalHistory } from './types'

export async function getAppInfo(): Promise<AppInfo> {
  return desktopInvoke<AppInfo>('get_app_info')
}

async function desktopInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    throw new Error('Desktop connection unavailable. Open Restna through the desktop app to check local storage.')
  }
  return args === undefined ? invoke<T>(command) : invoke<T>(command, args)
}

export const getDashboardState = () => desktopInvoke<DashboardState>('get_dashboard_state')
export const getSettings = () => desktopInvoke<SchedulerSettings>('get_settings')
export const updateSettings = (settings: SchedulerSettings) => desktopInvoke<DashboardState>('update_settings', { settings })
export const startBreak = (category: Category) => desktopInvoke<DashboardState>('start_break', { category })
export const completeBreak = (reminderId: string) => desktopInvoke<DashboardState>('complete_break', { reminderId })
export const skipBreak = (reminderId: string) => desktopInvoke<DashboardState>('skip_break', { reminderId })
export const snoozeBreak = (reminderId: string) => desktopInvoke<DashboardState>('snooze_break', { reminderId })
export const pauseReminders = (request: PauseRequest) => desktopInvoke<DashboardState>('pause_reminders', { request })
export const resumeReminders = () => desktopInvoke<DashboardState>('resume_reminders')
export const getHistory = (range: HistoryRange) => desktopInvoke<LocalHistory>('get_history', { range })
export const clearHistory = (confirmed: boolean) => desktopInvoke<void>('clear_history', { confirmed })
export const getNativeStatus = () => desktopInvoke<NativeStatus>('get_native_status')
export const setNotificationsEnabled = (enabled: boolean) => desktopInvoke<NativeStatus>('set_notifications_enabled', { enabled })
export const setAutostartEnabled = (enabled: boolean) => desktopInvoke<NativeStatus>('set_autostart_enabled', { enabled })
export const testNotification = () => desktopInvoke<NotificationAttempt>('test_notification')
export const openCurrentBreak = () => desktopInvoke<void>('open_current_break')
export const hideBreakWindow = () => desktopInvoke<void>('hide_break_window')
export const quitApp = () => desktopInvoke<void>('quit_app')

export async function subscribeScheduler<K extends keyof SchedulerEvents>(name: K, onEvent: (event: SchedulerEvents[K]) => void): Promise<UnlistenFn> {
  if (!isTauri()) throw new Error('Scheduler connection unavailable in browser preview.')
  return listen<SchedulerEvents[K]>(name, ({ payload }) => onEvent(payload))
}
