import { act, render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi } from 'vitest'
import { BreakPanel } from './BreakPanel'
import { useScheduler } from '../hooks/useScheduler'
import { completeBreak, skipBreak, snoozeBreak, startBreak, hideBreakWindow } from '../lib/api'
import { schedulerState } from '../test/schedulerFixtures'

vi.mock('../hooks/useScheduler', () => ({ useScheduler: vi.fn() }))
vi.mock('../lib/api', () => ({ completeBreak: vi.fn(), skipBreak: vi.fn(), snoozeBreak: vi.fn(), startBreak: vi.fn(), hideBreakWindow: vi.fn() }))
const id = '00000000-0000-0000-0000-000000000001'
beforeEach(() => {
  const state = schedulerState()
  state.openReminder = { id, category: 'eye', triggeredAtUtc: '2026-10-21T09:00:00Z', expiresAtUtc: '2026-10-21T09:05:00Z', startedAtUtc: null }
  vi.mocked(useScheduler).mockReturnValue({ status: 'ready', state })
  for (const command of [completeBreak, skipBreak, snoozeBreak, startBreak]) vi.mocked(command).mockResolvedValue(state)
  vi.mocked(hideBreakWindow).mockResolvedValue()
})

test.each([['Done', completeBreak], ['Snooze 5 min', snoozeBreak], ['Skip', skipBreak]] as const)('%s sends the current UUID to Rust', async (name, command) => {
  render(<BreakPanel />)
  await userEvent.click(screen.getByRole('button', { name }))
  expect(command).toHaveBeenCalledExactlyOnceWith(id)
  expect(hideBreakWindow).toHaveBeenCalledOnce()
})

test('water reminder uses gentle copy and Done never claims measured intake', async () => {
  const state = schedulerState()
  state.openReminder = { id, category: 'water', triggeredAtUtc: '2026-10-21T09:00:00Z', expiresAtUtc: '2026-10-21T09:05:00Z', startedAtUtc: null }
  vi.mocked(useScheduler).mockReturnValue({ status: 'ready', state })
  render(<BreakPanel />)
  expect(screen.getByRole('heading', { name: 'A moment for water.' })).toBeInTheDocument()
  expect(screen.getByText(/Consider a drink of water/)).toBeInTheDocument()
  await userEvent.click(screen.getByRole('button', { name: 'Done' }))
  expect(completeBreak).toHaveBeenCalledExactlyOnceWith(id)
  expect(hideBreakWindow).toHaveBeenCalledOnce()
})

test('Start does not complete; Esc and Close hide without recording outcomes', async () => {
  render(<BreakPanel />)
  const user = userEvent.setup()
  await user.click(screen.getByRole('button', { name: 'Start 20-second break' }))
  expect(startBreak).toHaveBeenCalledWith('eye')
  await user.keyboard('{Escape}')
  await user.click(screen.getByRole('button', { name: 'Close for now' }))
  expect(hideBreakWindow).toHaveBeenCalledTimes(2)
  expect(completeBreak).not.toHaveBeenCalled()
  expect(skipBreak).not.toHaveBeenCalled()
  expect(snoozeBreak).not.toHaveBeenCalled()
})

test('expired actions show the Rust error and pending actions cannot be submitted twice', async () => {
  let resolve!: (value: ReturnType<typeof schedulerState>) => void
  vi.mocked(completeBreak).mockReturnValueOnce(new Promise(yes => { resolve = yes }))
  render(<BreakPanel />)
  const user = userEvent.setup()
  await user.click(screen.getByRole('button', { name: 'Done' }))
  expect(screen.getByRole('button', { name: 'Done' })).toBeDisabled()
  await user.click(screen.getByRole('button', { name: 'Done' }))
  expect(completeBreak).toHaveBeenCalledOnce()
  await act(async () => resolve(schedulerState()))
  vi.mocked(skipBreak).mockRejectedValueOnce('Reminder is stale, expired, or no longer eligible.')
  await user.click(screen.getByRole('button', { name: 'Skip' }))
  expect(await screen.findByRole('alert')).toHaveTextContent('stale, expired')
})

test('no current reminder exposes no outcome buttons', () => {
  vi.mocked(useScheduler).mockReturnValue({ status: 'ready', state: schedulerState() })
  render(<BreakPanel />)
  expect(screen.getByRole('heading', { name: 'No break waiting.' })).toBeInTheDocument()
  expect(screen.queryByRole('button', { name: 'Done' })).not.toBeInTheDocument()
})

test('movement break offers a real start action and does not invent a countdown',async()=>{
  const state=schedulerState()
  state.openReminder={id,category:'movement',triggeredAtUtc:'2026-10-21T09:00:00Z',expiresAtUtc:'2026-10-21T09:05:00Z',startedAtUtc:null}
  vi.mocked(useScheduler).mockReturnValue({status:'ready',state})
  render(<BreakPanel />)
  await userEvent.click(screen.getByRole('button',{name:'Start 2-minute break'}))
  expect(startBreak).toHaveBeenCalledWith('movement')
  expect(screen.queryByRole('timer')).not.toBeInTheDocument()
})

test.each(['eye', 'movement'] as const)('%s Done hides only after Rust confirms the outcome', async category => {
  const state = schedulerState()
  state.openReminder = { id, category, triggeredAtUtc: '2026-10-21T09:00:00Z', expiresAtUtc: '2026-10-21T09:05:00Z', startedAtUtc: null }
  vi.mocked(useScheduler).mockReturnValue({ status: 'ready', state })
  let resolve!: (value: ReturnType<typeof schedulerState>) => void
  vi.mocked(completeBreak).mockReturnValueOnce(new Promise(yes => { resolve = yes }))
  render(<BreakPanel />)
  await userEvent.click(screen.getByRole('button', { name: 'Done' }))
  expect(hideBreakWindow).not.toHaveBeenCalled()
  await act(async () => resolve(schedulerState()))
  expect(hideBreakWindow).toHaveBeenCalledOnce()
})

test('failed completion stays visible; saved action with failed hide can retry Close without repeating the outcome', async () => {
  vi.mocked(completeBreak).mockRejectedValueOnce('Reminder expired.')
  render(<BreakPanel />)
  const user = userEvent.setup()
  await user.click(screen.getByRole('button', { name: 'Done' }))
  expect(await screen.findByRole('alert')).toHaveTextContent('Reminder expired.')
  expect(hideBreakWindow).not.toHaveBeenCalled()
  vi.mocked(hideBreakWindow).mockRejectedValueOnce('Window unavailable')
  await user.click(screen.getByRole('button', { name: 'Done' }))
  expect(await screen.findByRole('alert')).toHaveTextContent('Your action was saved')
  await user.click(screen.getByRole('button', { name: 'Close for now' }))
  expect(hideBreakWindow).toHaveBeenCalledTimes(2)
  expect(completeBreak).toHaveBeenCalledTimes(2)
})
