import { expect, test } from './fixtures'

test('shell loads, navigation responds, and no content overflows', async ({ page }) => {
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()) })
  await page.goto('/')
  await expect(page).toHaveTitle('Restnaa')
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('A little space to pause.')
  await expect(page.getByRole('alert').filter({ hasText: 'Desktop connection unavailable' })).toBeVisible()
  await expect(page.getByRole('alert').filter({ hasText: 'Scheduler unavailable' })).toBeVisible()
  await expect(page.locator('vite-error-overlay')).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'Welcome', exact: true })).toHaveCount(0)
  for (const label of ['Settings', 'History', 'Dashboard']) {
    await page.getByRole('button', { name: label, exact: true }).click()
    await expect(page.getByRole('button', { name: label, exact: true })).toHaveAttribute('aria-current', 'page')
  }
  expect(await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth)).toBe(false)
  expect(errors).toEqual([])
})

test('keyboard skip link and navigation are usable', async ({ page }) => {
  await page.goto('/')
  await expect(page.getByRole('heading', { level: 1 })).toBeVisible()
  await page.keyboard.press('Tab')
  await expect(page.getByRole('link', { name: 'Skip to content' })).toBeFocused()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('main')).toBeFocused()
  await page.getByRole('button', { name: 'Settings', exact: true }).focus()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('heading', { name: 'Settings', level: 1 })).toBeVisible()
})

test('break window route is readable and never fabricates a browser reminder', async ({ page }) => {
  await page.goto('/?window=break')
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('No break waiting.')
  await expect(page.getByRole('alert')).toContainText('Scheduler unavailable')
  await expect(page.getByRole('button', { name: 'Done', exact: true })).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'Close for now' })).toBeVisible()
  expect(await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth)).toBe(false)
})
