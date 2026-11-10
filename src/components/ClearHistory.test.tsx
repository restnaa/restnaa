import { render,screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi } from 'vitest'
import { ClearHistory } from './ClearHistory'
import { clearHistory } from '../lib/api'
vi.mock('../lib/api',()=>({clearHistory:vi.fn()}))
beforeEach(()=>{
  // jsdom has no modal implementation; real focus trapping is tested in Edge.
  HTMLDialogElement.prototype.showModal=function(){this.setAttribute('open','')}
  HTMLDialogElement.prototype.close=function(){this.removeAttribute('open');this.dispatchEvent(new Event('close'))}
})
test('opening or cancelling confirmation never deletes data',async()=>{
  render(<ClearHistory />)
  const trigger=screen.getByRole('button',{name:'Clear history…'})
  await userEvent.click(trigger)
  expect(screen.getByRole('button',{name:'Keep history'})).toHaveFocus()
  expect(clearHistory).not.toHaveBeenCalled()
  await userEvent.click(screen.getByRole('button',{name:'Keep history'}))
  expect(clearHistory).not.toHaveBeenCalled();expect(trigger).toHaveFocus()
})
test('failed confirmation preserves dialog and exposes backend error; retry is explicit',async()=>{
  vi.mocked(clearHistory).mockRejectedValueOnce('History is read-only.').mockResolvedValueOnce()
  render(<ClearHistory />)
  await userEvent.click(screen.getByRole('button',{name:'Clear history…'}))
  await userEvent.click(screen.getByRole('button',{name:'Permanently clear history'}))
  expect(await screen.findByRole('alert')).toHaveTextContent('History is read-only')
  expect(screen.getByRole('dialog')).toHaveAttribute('open')
  await userEvent.click(screen.getByRole('button',{name:'Permanently clear history'}))
  expect(await screen.findByRole('status')).toHaveTextContent('Local break history cleared')
  expect(clearHistory).toHaveBeenNthCalledWith(1,true);expect(clearHistory).toHaveBeenNthCalledWith(2,true)
})
