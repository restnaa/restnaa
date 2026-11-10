import type { Page } from '@playwright/test'
import { schedulerState } from '../../src/test/schedulerFixtures'
import { historyFixture } from '../../src/test/historyFixtures'

// This bridge exists only in browser tests. Production always invokes Rust.
export async function installDesktopFixture(page: Page, onboardingCompleted = true) {
  await page.addInitScript(({ initial, history }) => {
    type State = typeof initial
    const saved = localStorage.getItem('restnaa-test-state')
    const state: State = saved ? JSON.parse(saved) : initial
    const calls: { command: string; args: Record<string, unknown> }[] = []
    const callbacks = new Map<number, (event: unknown) => void>()
    const listeners = new Map<number, { name: string; callback: number }>()
    let sequence = 0
    let erased = false
    const native = { notificationEnabled: true, autostartEnabled: false, autostartRegistered: false, windowsPermission: 'allowed', attention: 'accepting', trayAvailable: true, lastAttempt: null as { atUtc: string; result: string; message: string } | null, warning: null }
    const emit = (name: string, payload: unknown) => { listeners.forEach(({ name: event, callback }, id) => { if (event === name) callbacks.get(callback)?.({ event: name, id, payload: structuredClone(payload) }) }) }
    const publish = () => { state.revision++; localStorage.setItem('restnaa-test-state',JSON.stringify(state)); emit('scheduler-state-changed',{kind:'state',state}) }
    const bridge = {
      transformCallback: (callback: (event: unknown) => void) => { const id=++sequence; callbacks.set(id,callback); return id },
      invoke: async (command: string, args: Record<string, unknown> = {}) => {
        calls.push({command,args})
        if(command==='plugin:event|listen'){const id=++sequence; listeners.set(id,{name:args.event as string,callback:args.handler as number});return id}
        if(command==='plugin:event|unlisten'){listeners.delete(args.eventId as number);return}
        if(command==='get_app_info')return {version:'0.1.0',storage:{status:'ready',schemaVersion:2}}
        if(command==='get_settings')return structuredClone(state.settings)
        if(command==='get_dashboard_state')return structuredClone(state)
        if(command==='update_settings'){state.settings=args.settings as State['settings'];publish();return structuredClone(state)}
        if(command==='get_native_status')return structuredClone(native)
        if(command==='set_autostart_enabled'){native.autostartEnabled=native.autostartRegistered=args.enabled as boolean;emit('native-status-changed',null);return structuredClone(native)}
        if(command==='set_notifications_enabled'){native.notificationEnabled=args.enabled as boolean;emit('native-status-changed',null);return structuredClone(native)}
        if(command==='test_notification'){native.lastAttempt={atUtc:'2026-10-21T12:00:00Z',result:'submittedUnconfirmed',message:'Submitted to Windows. Visible delivery is not confirmed.'};emit('native-status-changed',null);return native.lastAttempt}
        if(command==='get_history'){
          const result=structuredClone(history);result.range=args.range as typeof result.range
          if(args.range==='today')result.fromDate=result.throughDate
          if(erased){result.totalEvents=0;result.recent=[];result.categories.forEach(item=>{item.counts={completed:0,skipped:0,snoozed:0,unanswered:0}})}
          return result
        }
        if(command==='clear_history'){if(!args.confirmed)throw 'Confirmation required';erased=true;emit('history-changed',null);return}
        if(command==='pause_reminders'){const request=args.request as {kind:string;minutes?:number};state.status='paused';state.settings.pausedIndefinitely=request.kind==='manual';state.settings.pausedUntilUtc=request.kind==='manual'?null:'2026-10-22T00:00:00Z';publish();return structuredClone(state)}
        if(command==='resume_reminders'){state.status='active';state.settings.pausedIndefinitely=false;state.settings.pausedUntilUtc=null;publish();return structuredClone(state)}
        if(command==='start_break'){
          state.openReminder={id:'00000000-0000-0000-0000-000000000001',category:args.category as 'eye'|'movement',triggeredAtUtc:'2026-10-21T12:00:00Z',expiresAtUtc:'2026-10-21T12:05:00Z',startedAtUtc:'2026-10-21T12:00:00Z'};publish();return structuredClone(state)
        }
        if(['complete_break','skip_break','snooze_break'].includes(command)){state.openReminder=null;publish();return structuredClone(state)}
        if(['open_current_break','hide_break_window','quit_app'].includes(command))return
        throw new Error(`Unhandled test IPC: ${command}`)
      },
    }
    Object.assign(window,{isTauri:true,__TAURI_INTERNALS__:bridge,__TAURI_EVENT_PLUGIN_INTERNALS__:{unregisterListener:(_name:string,id:number)=>listeners.delete(id)},__restnaaTestCalls:calls})
  },{initial:{...schedulerState(),settings:{...schedulerState().settings,onboardingCompleted}},history:historyFixture()})
}

export async function commandCalls(page: Page, command: string) {
  return page.evaluate(name => (window as unknown as {__restnaaTestCalls:{command:string;args:Record<string,unknown>}[]}).__restnaaTestCalls.filter(call=>call.command===name),command)
}
