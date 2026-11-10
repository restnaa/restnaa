import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi } from 'vitest'
import { PauseControls } from './PauseControls'
import { pauseReminders, resumeReminders } from '../lib/api'
import { schedulerState } from '../test/schedulerFixtures'
vi.mock('../lib/api', () => ({ pauseReminders: vi.fn(), resumeReminders: vi.fn() }))
test.each([['0',{kind:'minutes',minutes:30}],['1',{kind:'minutes',minutes:60}],['2',{kind:'minutes',minutes:120}],['3',{kind:'untilTomorrow'}],['4',{kind:'manual'}]])('pause option %s uses the typed Rust request', async (choice, request) => {
  vi.mocked(pauseReminders).mockResolvedValue(schedulerState())
  render(<PauseControls settings={schedulerState().settings} paused={false} />)
  await userEvent.selectOptions(screen.getByLabelText('Pause for'),choice as string)
  await userEvent.click(screen.getByRole('button',{name:'Pause reminders'}))
  expect(pauseReminders).toHaveBeenCalledWith(request)
  expect(screen.getByRole('button',{name:'Resume reminders'})).toBeDisabled()
})
test('manual pause can resume and errors are visible',async()=>{
  vi.mocked(resumeReminders).mockRejectedValueOnce('Unable to persist resume.')
  render(<PauseControls settings={{...schedulerState().settings,pausedIndefinitely:true}} paused />)
  expect(screen.getByText('Paused until you choose Resume.')).toBeInTheDocument()
  await userEvent.click(screen.getByRole('button',{name:'Resume reminders'}))
  expect(await screen.findByRole('alert')).toHaveTextContent('Unable to persist resume')
})
