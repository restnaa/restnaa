import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi } from 'vitest'
import { ScheduleForm } from './ScheduleForm'
import { getSettings, updateSettings } from '../lib/api'
import { schedulerState } from '../test/schedulerFixtures'
vi.mock('../lib/api', () => ({ getSettings: vi.fn(), updateSettings: vi.fn() }))
beforeEach(() => { vi.mocked(getSettings).mockResolvedValue(schedulerState().settings); vi.mocked(updateSettings).mockResolvedValue(schedulerState()) })

test('water defaults to 60 minutes and saves an independent interval and opt-out', async () => {
  render(<ScheduleForm initial={schedulerState().settings} />)
  const user = userEvent.setup()
  expect(screen.getByRole('checkbox', { name: 'Water reminders' })).toBeChecked()
  expect(screen.getByLabelText('Water interval (minutes)')).toHaveValue(60)
  await user.clear(screen.getByLabelText('Water interval (minutes)'))
  await user.type(screen.getByLabelText('Water interval (minutes)'), '90')
  await user.click(screen.getByRole('checkbox', { name: 'Water reminders' }))
  await user.click(screen.getByRole('button', { name: 'Save settings' }))
  expect(updateSettings).toHaveBeenCalledWith(expect.objectContaining({ waterEnabled: false, waterIntervalMinutes: 90, eyeIntervalMinutes: 20, movementIntervalMinutes: 45 }))
})
test('saves real edits while preserving latest native and pause fields', async () => {
  const latest = { ...schedulerState().settings, notificationEnabled: false, pausedIndefinitely: true }
  vi.mocked(getSettings).mockResolvedValue(latest)
  render(<ScheduleForm initial={schedulerState().settings} />)
  const user = userEvent.setup()
  await user.clear(screen.getByLabelText('Eye interval (minutes)')); await user.type(screen.getByLabelText('Eye interval (minutes)'), '35')
  await user.click(screen.getByLabelText('Movement reminders'))
  await user.click(screen.getByLabelText('Saturday'))
  await user.click(screen.getByRole('button', { name: 'Save settings' }))
  expect(await screen.findByRole('status')).toHaveTextContent('Saved on this device')
  expect(updateSettings).toHaveBeenCalledWith(expect.objectContaining({ eyeIntervalMinutes: 35, movementEnabled: false, workDays: [0,1,2,3,4,5], notificationEnabled: false, pausedIndefinitely: true }))
})
test('empty day selection cannot mutate settings', async () => {
  render(<ScheduleForm initial={schedulerState().settings} />)
  for (const name of ['Monday','Tuesday','Wednesday','Thursday','Friday']) await userEvent.click(screen.getByLabelText(name))
  await userEvent.click(screen.getByRole('button', { name: 'Save settings' }))
  expect(await screen.findByRole('alert')).toHaveTextContent('at least one working day')
  expect(updateSettings).not.toHaveBeenCalled()
})
test('invalid intervals use browser validation and failed saves remain visible', async () => {
  render(<ScheduleForm initial={schedulerState().settings} />)
  const input = screen.getByLabelText('Eye interval (minutes)')
  await userEvent.clear(input); await userEvent.type(input, '9')
  await userEvent.click(screen.getByRole('button', { name: 'Save settings' }))
  expect(input).toBeInvalid(); expect(updateSettings).not.toHaveBeenCalled()
  await userEvent.clear(input); await userEvent.type(input, '20')
  vi.mocked(updateSettings).mockRejectedValueOnce('Storage is read-only.')
  await userEvent.click(screen.getByRole('button', { name: 'Save settings' }))
  expect(await screen.findByRole('alert')).toHaveTextContent('Storage is read-only')
})

test('automatic break windows default on and explicit off is saved without changing native consent', async () => {
  const latest={...schedulerState().settings,autostartEnabled:true,pausedIndefinitely:true}
  vi.mocked(getSettings).mockResolvedValue(latest)
  render(<ScheduleForm initial={schedulerState().settings} />)
  const control=screen.getByLabelText('Show break window when a reminder is due')
  expect(control).toBeChecked()
  await userEvent.click(control)
  await userEvent.click(screen.getByRole('button',{name:'Save settings'}))
  expect(await screen.findByRole('status')).toHaveTextContent('Saved on this device')
  expect(updateSettings).toHaveBeenCalledWith(expect.objectContaining({showBreakWindow:false,autostartEnabled:true,pausedIndefinitely:true}))
})
