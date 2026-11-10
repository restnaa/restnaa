import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi } from 'vitest'
import App from './App'
import { getAppInfo, getDashboardState, getSettings, getHistory, updateSettings, subscribeScheduler } from './lib/api'
import { historyFixture } from './test/historyFixtures'
import { schedulerState } from './test/schedulerFixtures'
vi.mock('./hooks/useNativeStatus', () => ({ useNativeStatus: () => ({ status: null, error: null, refresh: vi.fn() }) }))
vi.mock('./components/NativePreferences', () => ({ NativePreferences: () => <p>Native settings test boundary</p> }))

vi.mock('./lib/api', () => ({ getAppInfo: vi.fn(), getSettings: vi.fn(), getHistory: vi.fn(), updateSettings: vi.fn(), getDashboardState: vi.fn(), subscribeScheduler: vi.fn() }))

beforeEach(() => {
  vi.mocked(getSettings).mockResolvedValue(schedulerState().settings)
  vi.mocked(getHistory).mockImplementation(async range => historyFixture(range))
  vi.mocked(updateSettings).mockResolvedValue(schedulerState())
  vi.mocked(getDashboardState).mockResolvedValue(schedulerState())
  vi.mocked(subscribeScheduler).mockResolvedValue(vi.fn())
  vi.mocked(getAppInfo).mockResolvedValue({ version: '0.1.0', storage: { status: 'ready', schemaVersion: 1 } })
})

test('renders real backend readiness and honest foundation state', async () => {
  render(<App />)
  expect(await screen.findByText('Ready · schema 1')).toBeInTheDocument()
  expect(screen.getByRole('button', { name: 'Settings' })).toBeInTheDocument()
  expect(await screen.findByText('Active')).toBeInTheDocument()
  expect(screen.getByText('Next eye break')).toBeInTheDocument()
  expect(screen.getByText('Next movement break')).toBeInTheDocument()
  expect(screen.getByRole('button', { name: 'Take eye break' })).toBeEnabled()
})

test('all navigation works and focus moves to the main content', async () => {
  const user = userEvent.setup()
  render(<App />)
  for (const [name, heading] of [['Settings', 'Settings'], ['History', 'History'], ['Dashboard', 'A little space to pause.']]) {
    const button = screen.getByRole('button', { name })
    await user.click(button)
    expect(screen.getByRole('heading', { name: heading, level: 1 })).toBeInTheDocument()
    expect(button).toHaveAttribute('aria-current', 'page')
    expect(screen.getByRole('main')).toHaveFocus()
  }
})

test('setup is part of Settings and there is no Welcome navigation', async () => {
  render(<App />)
  await userEvent.click(screen.getByRole('button', { name: 'Settings' }))
  expect(screen.queryByRole('button', { name: 'Welcome' })).not.toBeInTheDocument()
  expect(await screen.findByRole('button', { name: 'Save settings' })).toBeInTheDocument()
})

test('storage errors remain visible without claiming readiness', async () => {
  vi.mocked(getAppInfo).mockResolvedValue({ version: '0.1.0', storage: { status: 'unavailable', message: 'Storage could not be opened. Existing data has not been erased.' } })
  render(<App />)
  expect(await screen.findByRole('alert')).toHaveTextContent('Existing data has not been erased.')
  expect(screen.queryByText(/Ready · schema/)).not.toBeInTheDocument()
})

test('invoke failure shows a connection error rather than mock data', async () => {
  vi.mocked(getAppInfo).mockRejectedValue(new Error('IPC failed'))
  render(<App />)
  expect(await screen.findByRole('alert')).toHaveTextContent('Desktop connection unavailable')
})

 test('first launch opens setup and finishes only after persisted success', async () => {
  const state = schedulerState(); state.settings.onboardingCompleted = false
  vi.mocked(getSettings).mockResolvedValue(state.settings)
  vi.mocked(getDashboardState).mockResolvedValue(state)
  render(<App />)
  expect(await screen.findByRole('heading', { name: 'Settings', level: 1 })).toBeInTheDocument()
  await userEvent.click(await screen.findByRole('button', { name: 'Finish setup' }))
  expect(updateSettings).toHaveBeenCalledWith(expect.objectContaining({ onboardingCompleted: true, autostartEnabled: false }))
  expect(await screen.findByRole('heading', { name: 'A little space to pause.' })).toBeInTheDocument()
})
