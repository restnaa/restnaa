import { expect, test } from './fixtures'
import { installDesktopFixture, commandCalls } from './desktopFixture'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { mkdirSync } from 'node:fs'

test('reference layout, native minimum and high DPI reflow without overlap', async ({page, browser}) => {
  await installDesktopFixture(page); await page.goto('/')
  for (const width of [1448, 1200, 720, 390]) {
    await page.setViewportSize({width,height:width === 1448 ? 1086 : 800})
    for (const label of ['Dashboard','Settings','History']) {
      await page.getByRole('button',{name:label,exact:true}).click()
      await expect(page.getByRole('heading',{level:1})).toBeVisible()
      expect(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),`${label} at ${width}`).toBe(false)
      if ((width === 1448 || width === 390) && test.info().project.name === 'desktop') {
        const folder=join(tmpdir(),'pausepal-redesign');mkdirSync(folder,{recursive:true})
        await page.evaluate(()=>{(document.activeElement as HTMLElement)?.blur();window.scrollTo(0,0)})
        await page.screenshot({path:join(folder,`${label.toLowerCase()}${width === 390 ? '-compact' : ''}.png`),fullPage:true})
      }
      if (label === 'Dashboard' && width === 1200) {
        await expect(page.getByRole('button',{name:'Take eye break',exact:true})).toBeInViewport()
        await expect(page.getByRole('button',{name:'Resume reminders'})).toBeInViewport()
      }
    }
  }
  const context=await browser.newContext({viewport:{width:1200,height:820},deviceScaleFactor:2})
  const hi=await context.newPage()
  // Copy the same exact bundled-asset routing used by the shared fixture.
  await hi.route('http://127.0.0.1:1420/assets/**',async route=>{
    const {basename,resolve}=await import('node:path')
    await route.fulfill({path:resolve('dist/assets',basename(new URL(route.request().url()).pathname))})
  })
  await installDesktopFixture(hi);await hi.goto('http://127.0.0.1:1420')
  await expect(hi.getByRole('button',{name:'Take movement break',exact:true})).toBeVisible()
  expect(await hi.evaluate(()=>document.documentElement.scrollWidth>innerWidth)).toBe(false)
  await context.close()
})

test('history day disclosure, native toggles and Quit retain actual command paths',async({page})=>{
  await installDesktopFixture(page);await page.goto('/')
  await page.getByRole('button',{name:'History',exact:true}).click()
  const day=page.locator('.activity-group').first()
  await expect(day.locator('.event-list')).toContainText('Snoozed')
  await day.locator(':scope > summary').click();await expect(day.locator('.event-list')).not.toBeVisible()
  await day.locator(':scope > summary').focus();await page.keyboard.press('Enter');await expect(day.locator('.event-list')).toBeVisible()
  await page.getByRole('button',{name:'Settings',exact:true}).click()
  await page.getByLabel('Enable notifications').uncheck()
  await expect(page.getByRole('button',{name:'Send test reminder'})).toBeDisabled()
  await page.getByLabel('Start on login').check()
  expect(await commandCalls(page,'set_autostart_enabled')).toMatchObject([{args:{enabled:true}}])
  await page.getByRole('button',{name:'Refresh diagnostics'}).click()
  await page.getByRole('button',{name:'Quit Restnaa completely'}).click()
  expect(await commandCalls(page,'quit_app')).toHaveLength(1)
})

test('bundled landscape loads under the actual release CSP without inline data URLs',async({page})=>{
  const {readFileSync}=await import('node:fs')
  const csp=JSON.parse(readFileSync('src-tauri/tauri.conf.json','utf8')).app.security.csp
  const errors:string[]=[];page.on('console',message=>{if(message.type()==='error')errors.push(message.text())})
  await page.route('http://127.0.0.1:1420/',async route=>{
    const response=await route.fetch()
    await route.fulfill({response,headers:{...response.headers(),'content-security-policy':csp}})
  })
  await installDesktopFixture(page);await page.goto('/')
  await expect(page.getByRole('heading',{level:1})).toBeVisible()
  const background=await page.locator('.page-header').evaluate(element=>getComputedStyle(element,'::before').backgroundImage)
  expect(background).toContain('night-landscape')
  expect(background).not.toContain('data:')
  const loaded=await page.evaluate(async background=>{
    const url=background.slice(5,-2)
    return new Promise<boolean>(resolve=>{const image=new Image();image.onload=()=>resolve(image.naturalWidth>0);image.onerror=()=>resolve(false);image.src=url})
  },background)
  expect(loaded).toBe(true);expect(errors).toEqual([])
})
