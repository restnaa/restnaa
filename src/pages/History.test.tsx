import { render,screen } from '@testing-library/react'
import { vi } from 'vitest'
import { History } from './History'
import { useHistory } from '../hooks/useHistory'
import { historyFixture } from '../test/historyFixtures'
vi.mock('../hooks/useHistory',()=>({useHistory:vi.fn()}))
test('all four outcome types and actual category counts remain distinct',()=>{
  vi.mocked(useHistory).mockReturnValue({view:{status:'ready',history:historyFixture()},refresh:vi.fn()})
  render(<History />)
  for(const name of ['Completed','Skipped','Snoozed','Unanswered']) expect(screen.getAllByText(name)[0]).toBeInTheDocument()
  expect(screen.getByText('14 recorded outcomes in this period.')).toBeInTheDocument()
  expect(screen.getByText('Snoozed',{selector:'span'})).toBeInTheDocument()
})
test('empty history and history failure are different states',()=>{
  const history={...historyFixture(),totalEvents:0,recent:[]}
  vi.mocked(useHistory).mockReturnValue({view:{status:'ready',history},refresh:vi.fn()})
  const {rerender}=render(<History />)
  expect(screen.getByText(/No recorded outcomes in this period/)).toBeInTheDocument()
  vi.mocked(useHistory).mockReturnValue({view:{status:'error',message:'Local history unavailable.'},refresh:vi.fn()})
  rerender(<History />)
  expect(screen.getByRole('alert')).toHaveTextContent('unavailable')
  expect(screen.queryByRole('table')).not.toBeInTheDocument()
})

 test('day headers use full totals and do not invent exercise descriptions', () => {
  const history = historyFixture()
  vi.mocked(useHistory).mockReturnValue({view:{status:'ready',history},refresh:vi.fn()})
  const {container} = render(<History />)
  expect(container.querySelector('.day-counts')).toHaveTextContent('2')
  expect(container.querySelector('.event-list')).toHaveTextContent('Snoozed')
  expect(screen.queryByText(/Stood and stretched/)).not.toBeInTheDocument()
  expect(container.querySelectorAll('.event-list li')).toHaveLength(history.recent.length)
})
 test('truncated history explains days whose event details are outside the cap', () => {
  const history = {...historyFixture(), recent: [], recentTruncated: true}
  vi.mocked(useHistory).mockReturnValue({view:{status:'ready',history},refresh:vi.fn()})
  render(<History />)
  expect(screen.getByText('Earlier outcomes are outside the latest 100 shown.')).toBeInTheDocument()
  expect(screen.getByText(/Showing the latest 100/)).toBeInTheDocument()
})

test('SQLite chronological days render newest first with today expanded without mutating backend data', () => {
  const history=historyFixture()
  history.days=[{...history.days[0],date:'2026-10-20'},history.days[0]]
  vi.mocked(useHistory).mockReturnValue({view:{status:'ready',history},refresh:vi.fn()})
  const {container}=render(<History />)
  const groups=container.querySelectorAll('.activity-group')
  expect(groups[0].querySelector('summary time')).toHaveAttribute('datetime','2026-10-21')
  expect(groups[0]).toHaveAttribute('open')
  expect(groups[1]).not.toHaveAttribute('open')
  expect(history.days[0].date).toBe('2026-10-20')
})
