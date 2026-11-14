import type { DashboardState } from '../lib/types'

export function schedulerState(revision = 1): DashboardState {
  return {
    revision, status: 'active',
    settings: { eyeEnabled: true, movementEnabled: true, eyeIntervalMinutes: 20, movementIntervalMinutes: 45, waterEnabled: true, waterIntervalMinutes: 60, workDays: [0,1,2,3,4], workStartLocal: '09:00:00', workEndLocal: '18:00:00', quietMode: false, pausedUntilUtc: null, pausedIndefinitely: false, notificationEnabled: true, autostartEnabled: false, showBreakWindow: true, onboardingCompleted: true },
    reminders: [
      { category: 'eye', nextDueAtUtc: '2026-10-21T09:20:00Z', openReminderId: null, status: 'scheduled' },
      { category: 'movement', nextDueAtUtc: '2026-10-21T09:45:00Z', openReminderId: null, status: 'scheduled' },
      { category: 'water', nextDueAtUtc: '2026-10-21T10:00:00Z', openReminderId: null, status: 'scheduled' },
    ], openReminder: null,
  }
}
