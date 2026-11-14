import { deadlineLabel, showDeadline } from './deadlines'
import { schedulerState } from '../test/schedulerFixtures'
const now = Date.parse('2026-10-21T09:08:01Z')

test('water eligibility is independent of movement preferences', () => {
  const state = schedulerState()
  state.settings.movementEnabled = false
  expect(deadlineLabel(state, state.reminders[2], now)).not.toBe('Disabled')
  expect(showDeadline(state, state.reminders[2])).toBe(true)
  state.settings.waterEnabled = false
  expect(deadlineLabel(state, state.reminders[2], now)).toBe('Disabled')
  expect(showDeadline(state, state.reminders[2])).toBe(false)
})
test('relative text uses the actual independent Rust deadlines', () => {
  const state = schedulerState()
  expect(deadlineLabel(state, state.reminders[0], now)).toBe('Due in 12 min')
  expect(deadlineLabel(state, state.reminders[1], now)).toBe('Due in 37 min')
  expect(deadlineLabel(state, state.reminders[0], now + 3600000)).toBe('Awaiting scheduler')
})
test.each(['paused','quiet'] as const)('suppressed %s timestamps do not claim an upcoming countdown', status => {
  const state = schedulerState(); state.status = status
  expect(showDeadline(state, state.reminders[0])).toBe(false)
  expect(deadlineLabel(state, state.reminders[0], now)).not.toMatch(/Due in/)
})
test('an open reminder or disabled category never invents time remaining', () => {
  const state = schedulerState(); state.settings.eyeEnabled = false
  expect(deadlineLabel(state, state.reminders[0], now)).toBe('Disabled')
  state.openReminder = {id:'id',category:'movement',triggeredAtUtc:'',expiresAtUtc:'',startedAtUtc:null}
  expect(showDeadline(state, state.reminders[1])).toBe(false)
  expect(deadlineLabel(state, state.reminders[1], now)).toBe('Reminder open')
})
