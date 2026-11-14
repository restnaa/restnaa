import type { DashboardState, ReminderState } from './types'
export const reminderEnabled = (state: DashboardState, reminder: ReminderState) => state.settings[({ eye: 'eyeEnabled', movement: 'movementEnabled', water: 'waterEnabled' } as const)[reminder.category]]
// Presentation only. Eligibility and reminder transitions remain in Rust.
export function deadlineLabel(state: DashboardState, reminder: ReminderState, now: number): string {
  if (!reminderEnabled(state, reminder)) return 'Disabled'
  if (state.status === 'paused') return 'After you resume'
  if (state.status === 'quiet') return 'After quiet mode ends'
  if (reminder.openReminderId || state.openReminder?.category === reminder.category) return 'Reminder open'
  const delta = Date.parse(reminder.nextDueAtUtc) - now
  if (!Number.isFinite(delta)) return 'Checking schedule'
  if (delta <= 0) return state.status === 'outsideWorkHours' ? 'Outside work hours' : 'Awaiting scheduler'
  const minutes = Math.ceil(delta / 60000)
  if (minutes >= 1440) return `In ${Math.ceil(minutes / 1440)} days`
  if (minutes >= 60) return `In ${Math.floor(minutes / 60)}h ${minutes % 60}m`
  return `Due in ${minutes} min`
}
export function showDeadline(state: DashboardState, reminder: ReminderState): boolean {
  return reminderEnabled(state, reminder)
    && state.status !== 'paused' && state.status !== 'quiet' && !reminder.openReminderId && state.openReminder?.category !== reminder.category
}
