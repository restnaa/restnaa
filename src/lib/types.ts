export interface AppInfo {
  version: string
  storage: { status: 'ready'; schemaVersion: number } | { status: 'unavailable'; message: string }
}

export type AppInfoState =
  | { status: 'loading' }
  | { status: 'loaded'; info: AppInfo }
  | { status: 'error'; message: string }

export type Page = 'dashboard' | 'settings' | 'history'

export type Category = 'eye' | 'movement'
export type Outcome = 'completed' | 'skipped' | 'snoozed' | 'unanswered'
export interface SchedulerSettings {
  eyeEnabled: boolean
  movementEnabled: boolean
  eyeIntervalMinutes: number
  movementIntervalMinutes: number
  /** Monday = 0, Sunday = 6. Times use HH:mm:ss. */
  workDays: number[]
  workStartLocal: string
  workEndLocal: string
  quietMode: boolean
  pausedUntilUtc: string | null
  pausedIndefinitely: boolean
  notificationEnabled: boolean
  autostartEnabled: boolean
  showBreakWindow: boolean
  onboardingCompleted: boolean
}
export interface ReminderState {
  category: Category
  nextDueAtUtc: string
  openReminderId: string | null
  status: 'scheduled' | 'open' | 'disabled' | 'paused' | 'outsideWorkHours' | 'quiet'
}
export interface OpenReminder {
  id: string
  category: Category
  triggeredAtUtc: string
  expiresAtUtc: string
  startedAtUtc: string | null
}
export interface BreakEvent {
  id: string
  category: Category
  outcome: Outcome
  triggeredAtUtc: string
  actedAtUtc: string
  durationSeconds: number | null
  createdAtUtc: string
}
export interface DashboardState {
  revision: number
  status: 'active' | 'paused' | 'outsideWorkHours' | 'quiet' | 'disabled'
  settings: SchedulerSettings
  reminders: [ReminderState, ReminderState]
  openReminder: OpenReminder | null
}
export type PauseRequest = { kind: 'minutes'; minutes: 30 | 60 | 120 } | { kind: 'untilTomorrow' } | { kind: 'manual' }
export type SchedulerStateEvent = { kind: 'state'; state: DashboardState } | { kind: 'error'; message: string }
export interface SchedulerEvents {
  'scheduler-state-changed': SchedulerStateEvent
  'reminder-due': OpenReminder
  'break-event-recorded': BreakEvent
  'settings-updated': SchedulerSettings
  'native-status-changed': null
  'history-changed': null
  navigate: Page
}

export type HistoryRange = 'today' | 'sevenDays'
export interface OutcomeCounts { completed: number; skipped: number; snoozed: number; unanswered: number }
export interface CategoryCounts { category: Category; counts: OutcomeCounts }
export interface LocalHistory {
  range: HistoryRange
  fromDate: string
  throughDate: string
  asOfUtc: string
  categories: [CategoryCounts, CategoryCounts]
  days: { date: string; categories: [CategoryCounts, CategoryCounts] }[]
  recent: BreakEvent[]
  totalEvents: number
  recentTruncated: boolean
}

export interface NotificationAttempt {
  atUtc: string
  result: 'disabledInApp' | 'deniedByWindows' | 'suppressedByAttention' | 'submittedUnconfirmed' | 'preparationFailed'
  message: string
}
export interface NativeStatus {
  notificationEnabled: boolean
  autostartEnabled: boolean
  autostartRegistered: boolean | null
  windowsPermission: 'allowed' | 'disabledForApplication' | 'disabledForUser' | 'disabledByPolicy' | 'disabledByManifest' | 'unknown'
  attention: 'accepting' | 'away' | 'busy' | 'fullscreen' | 'presentation' | 'quietTime' | 'app' | 'unknown'
  trayAvailable: boolean
  lastAttempt: NotificationAttempt | null
  warning: string | null
}
