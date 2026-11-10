import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi } from 'vitest'
import { NativePreferences } from './NativePreferences'
import { useNativeStatus } from '../hooks/useNativeStatus'
import { setAutostartEnabled, setNotificationsEnabled, testNotification, quitApp } from '../lib/api'
import type { NativeStatus } from '../lib/types'

vi.mock('../hooks/useNativeStatus', () => ({ useNativeStatus: vi.fn() }))
vi.mock('../lib/api', () => ({ setAutostartEnabled: vi.fn(), setNotificationsEnabled: vi.fn(), testNotification: vi.fn(), quitApp: vi.fn() }))
const status: NativeStatus = { notificationEnabled: true, autostartEnabled: false, autostartRegistered: false, windowsPermission: 'allowed', attention: 'unknown', trayAvailable: true, lastAttempt: null, warning: null }
const refresh = vi.fn<() => Promise<void>>()
beforeEach(() => {
  refresh.mockResolvedValue()
  vi.mocked(useNativeStatus).mockReturnValue({ status, error: null, refresh })
  vi.mocked(setAutostartEnabled).mockResolvedValue({ ...status, autostartEnabled: true, autostartRegistered: true })
  vi.mocked(setNotificationsEnabled).mockResolvedValue(status)
  vi.mocked(quitApp).mockResolvedValue()
  vi.mocked(testNotification).mockResolvedValue({ atUtc: '2026-10-21T09:00:00Z', result: 'submittedUnconfirmed', message: 'Submitted to Windows. Visible delivery is not confirmed.' })
})

test('autostart defaults off and changes only after explicit checkbox interaction', async () => {
  render(<NativePreferences />)
  expect(screen.getByRole('checkbox', { name: 'Start on login' })).not.toBeChecked()
  expect(setAutostartEnabled).not.toHaveBeenCalled()
  await userEvent.click(screen.getByRole('checkbox', { name: 'Start on login' }))
  expect(setAutostartEnabled).toHaveBeenCalledExactlyOnceWith(true)
  expect(refresh).toHaveBeenCalled()
})

test('startup errors stay visible without pretending the preference changed', async () => {
  vi.mocked(setAutostartEnabled).mockRejectedValueOnce('Could not change Windows startup registration.')
  render(<NativePreferences />)
  await userEvent.click(screen.getByRole('checkbox', { name: 'Start on login' }))
  expect(await screen.findByRole('alert')).toHaveTextContent('Could not change Windows startup')
  expect(screen.getByRole('checkbox', { name: 'Start on login' })).not.toBeChecked()
})

test('notification test uses the native command and explains unconfirmed display', async () => {
  vi.mocked(useNativeStatus).mockReturnValue({ status: { ...status, lastAttempt: { atUtc: 'now', result: 'submittedUnconfirmed', message: 'Submitted to Windows. Visible delivery is not confirmed.' } }, error: null, refresh })
  render(<NativePreferences />)
  await userEvent.click(screen.getByRole('button', { name: 'Send test reminder' }))
  expect(testNotification).toHaveBeenCalledOnce()
  expect(screen.getByRole('status')).toHaveTextContent('Visible delivery is not confirmed')
  expect(screen.getByText(/Do not disturb\/Focus Assist may hide banners/)).toBeInTheDocument()
})

test('disabled notifications, unreadable startup state, and explicit quit are handled', async () => {
  vi.mocked(useNativeStatus).mockReturnValue({ status: { ...status, notificationEnabled: false, autostartRegistered: null, windowsPermission: 'disabledByPolicy' }, error: null, refresh })
  render(<NativePreferences />)
  expect(screen.getByRole('button', { name: 'Send test reminder' })).toBeDisabled()
  expect(screen.getByRole('checkbox', { name: 'Start on login' })).toBeDisabled()
  expect(screen.getByText('Disabled by Windows or policy')).toBeInTheDocument()
  await userEvent.click(screen.getByRole('button', { name: 'Quit Restna completely' }))
  expect(quitApp).toHaveBeenCalledOnce()
})

test('diagnostics or storage failure cannot hide explicit Quit',async()=>{
  vi.mocked(useNativeStatus).mockReturnValue({status:null,error:'Storage is unavailable.',refresh})
  render(<NativePreferences />)
  expect(screen.getByRole('alert')).toHaveTextContent('Storage is unavailable')
  await userEvent.click(screen.getByRole('button',{name:'Quit Restna completely'}))
  expect(quitApp).toHaveBeenCalledOnce()
})
