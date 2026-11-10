import { act, renderHook, waitFor } from '@testing-library/react'
import { vi } from 'vitest'
import { useHistory } from './useHistory'
import { getHistory, subscribeScheduler } from '../lib/api'
import { historyFixture } from '../test/historyFixtures'
import type { LocalHistory } from '../lib/types'
vi.mock('../lib/api',()=>({getHistory:vi.fn(),subscribeScheduler:vi.fn()}))
test('outcomes and deletion refresh history; stale responses cannot restore deleted counts',async()=>{
  const callbacks: (()=>void)[]=[]
  const stops=[vi.fn(),vi.fn()]
  vi.mocked(subscribeScheduler).mockImplementation(async(_name,callback)=>{callbacks.push(callback as ()=>void);return stops[callbacks.length-1]})
  let resolve!: (history:LocalHistory)=>void
  vi.mocked(getHistory).mockReturnValueOnce(new Promise(yes=>{resolve=yes})).mockResolvedValue(historyFixture())
  const {result,unmount}=renderHook(()=>useHistory('sevenDays'))
  await waitFor(()=>expect(getHistory).toHaveBeenCalledOnce())
  act(()=>callbacks[1]())
  await waitFor(()=>expect(result.current.view.status).toBe('ready'))
  await act(async()=>resolve({...historyFixture(),totalEvents:999}))
  expect(result.current.view).toMatchObject({status:'ready',history:{totalEvents:14}})
  act(()=>callbacks[0]())
  await waitFor(()=>expect(getHistory).toHaveBeenCalledTimes(3))
  unmount();stops.forEach(stop=>expect(stop).toHaveBeenCalledOnce())
})
test('period changes reload and failures never render fake zero counts',async()=>{
  vi.mocked(subscribeScheduler).mockResolvedValue(vi.fn())
  vi.mocked(getHistory).mockResolvedValueOnce(historyFixture())
  const {result,rerender}=renderHook(({range})=>useHistory(range),{initialProps:{range:'sevenDays' as 'today'|'sevenDays'}})
  await waitFor(()=>expect(result.current.view.status).toBe('ready'))
  vi.mocked(getHistory).mockRejectedValueOnce(new Error('denied'))
  rerender({range:'today'})
  await waitFor(()=>expect(result.current.view.status).toBe('error'))
  expect(result.current.view).not.toHaveProperty('history')
})
