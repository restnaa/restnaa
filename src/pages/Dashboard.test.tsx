import { render, screen } from '@testing-library/react'
import { vi } from 'vitest'
import { Dashboard } from './Dashboard'
import { useScheduler } from '../hooks/useScheduler'
import { useHistory } from '../hooks/useHistory'

vi.mock('../hooks/useScheduler', () => ({ useScheduler: vi.fn() }))
vi.mock('../hooks/useHistory', () => ({ useHistory: vi.fn() }))
vi.mock('../hooks/useNativeStatus', () => ({ useNativeStatus: () => ({ status: null, error: null, refresh: vi.fn() }) }))
vi.mock('../lib/api', () => ({ startBreak: vi.fn(), openCurrentBreak: vi.fn() }))

const appInfo = { status: 'loaded' as const, info: { version: '0.1.4', storage: { status: 'ready' as const, schemaVersion: 3 } } }

beforeEach(() => {
  vi.mocked(useScheduler).mockReturnValue({ status: 'loading' })
  vi.mocked(useHistory).mockReturnValue({ view: { status: 'loading' }, refresh: vi.fn() })
})

// The loading labels are plain static copy, so a mis-encoded ellipsis here is a
// visible defect with no functional signal. Pin the exact characters.
test('loading status copy uses a real ellipsis and no mis-decoded text', () => {
  render(<Dashboard appInfo={appInfo} navigate={vi.fn()} />)
  const scheduler = screen.getByText(/Connecting to the Rust scheduler/)
  const history = screen.getByText(/Reading local history/)
  for (const line of [scheduler, history]) {
    expect(line.textContent).toContain('…')
    expect(line.textContent).not.toContain('â€¦')
  }
  expect(scheduler.textContent).toBe('Connecting to the Rust scheduler…')
  expect(history.textContent).toBe('Reading local history…')
})
