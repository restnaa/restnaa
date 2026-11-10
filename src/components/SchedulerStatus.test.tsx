import { render, screen } from '@testing-library/react'
import { vi } from 'vitest'
import { SchedulerStatus } from './SchedulerStatus'
import { useScheduler } from '../hooks/useScheduler'
import { schedulerState } from '../test/schedulerFixtures'

vi.mock('../hooks/useScheduler', () => ({ useScheduler: vi.fn() }))

test.each(['paused', 'quiet'] as const)('suppressed %s schedule does not present stale deadlines as upcoming breaks', status => {
  const state = schedulerState()
  state.status = status
  vi.mocked(useScheduler).mockReturnValue({ status: 'ready', state })
  const { container } = render(<SchedulerStatus />)
  expect(container.querySelector('time')).toBeNull()
  expect(screen.getAllByText(status === 'paused' ? 'After you resume' : 'After quiet mode ends')).toHaveLength(2)
})

test('open and disabled categories do not show invented countdowns', () => {
  const state = schedulerState()
  state.reminders[0].openReminderId = 'id'
  state.settings.movementEnabled = false
  vi.mocked(useScheduler).mockReturnValue({ status: 'ready', state })
  const { container } = render(<SchedulerStatus />)
  expect(screen.getByText('Reminder open')).toBeInTheDocument()
  expect(screen.getByText('Disabled')).toBeInTheDocument()
  expect(container.querySelector('time')).toBeNull()
})
