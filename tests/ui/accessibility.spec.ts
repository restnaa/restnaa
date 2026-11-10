import { expect, test } from './fixtures'
import AxeBuilder from '@axe-core/playwright'
import { installDesktopFixture } from './desktopFixture'

test('release workflows have no detected WCAG A/AA violations',async({page})=>{
  test.setTimeout(90000)
  await installDesktopFixture(page);await page.goto('/')
  await expect(page.getByRole('button',{name:'Take eye break'})).toBeVisible()
  for(const label of ['Dashboard','Settings','History']){
    await page.getByRole('button',{name:label,exact:true}).click()
    await expect(page.getByRole('heading',{level:1})).toBeVisible()
    const result=await new AxeBuilder({page}).withTags(['wcag2a','wcag2aa','wcag21a','wcag21aa']).analyze()
    expect(result.violations.map(item=>({id:item.id,nodes:item.nodes.map(node=>node.target)})),label).toEqual([])
  }
  await page.getByRole('button',{name:'Settings',exact:true}).click()
  await page.getByRole('button',{name:/^Clear history/}).click()
  const modal=await new AxeBuilder({page}).withTags(['wcag2a','wcag2aa','wcag21a','wcag21aa']).analyze()
  expect(modal.violations.map(item=>({id:item.id,nodes:item.nodes.map(node=>node.target)}))).toEqual([])
  await page.keyboard.press('Escape')
})

test('forced colors and zoom preserve keyboard controls and visible focus',async({page})=>{
  // At 200% zoom, use a supported desktop width. 390px / 2 is
  // below the app's 320 CSS-pixel minimum and its native 720px window minimum.
  await page.setViewportSize({width:1280,height:1000})
  await installDesktopFixture(page);await page.emulateMedia({forcedColors:'active',reducedMotion:'reduce'});await page.goto('/')
  await page.getByRole('button',{name:'Settings',exact:true}).click()
  await page.evaluate(()=>{document.documentElement.style.zoom='2'})
  const interval=page.getByLabel('Eye interval (minutes)');await interval.focus()
  await expect(interval).toBeFocused()
  expect(await interval.evaluate(element=>getComputedStyle(element).outlineStyle)).not.toBe('none')
  await page.getByRole('button',{name:/^Clear history/}).click()
  await expect(page.getByRole('button',{name:'Keep history'})).toBeFocused()
  await page.keyboard.press('Tab');await expect(page.getByRole('button',{name:'Permanently clear history'})).toBeFocused()
  await page.keyboard.press('Tab');await expect(page.getByRole('button',{name:'Keep history'})).toBeFocused()
  await page.keyboard.press('Escape')
  expect(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth)).toBe(false)
})

test('break panel remains accessible for both categories',async({page})=>{
  await installDesktopFixture(page);await page.goto('/')
  await page.getByRole('button',{name:'Take eye break'}).click();await page.goto('/?window=break')
  await expect(page.getByRole('heading',{name:'A moment for your eyes.'})).toBeVisible()
  const result=await new AxeBuilder({page}).withTags(['wcag2a','wcag2aa','wcag21a','wcag21aa']).analyze()
  expect(result.violations.map(item=>item.id)).toEqual([])
  await page.getByRole('button',{name:'Done',exact:true}).click()
  await page.goto('/');await page.getByRole('button',{name:'Take movement break'}).click();await page.goto('/?window=break')
  await expect(page.getByRole('heading',{name:'Make room to move.'})).toBeVisible()
  const movement=await new AxeBuilder({page}).withTags(['wcag2a','wcag2aa','wcag21a','wcag21aa']).analyze()
  expect(movement.violations.map(item=>item.id)).toEqual([])
})
