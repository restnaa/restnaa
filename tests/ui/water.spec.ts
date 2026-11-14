import { expect, test } from './fixtures'
import { commandCalls, installDesktopFixture } from './desktopFixture'

test('water defaults, interval edits and opt-out persist through typed settings', async ({ page }) => {
  await installDesktopFixture(page)
  await page.goto('/')
  await expect(page.getByRole('heading', { name: 'Water reminder', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Settings', exact: true }).click()
  await expect(page.getByLabel('Water reminders', { exact: true })).toBeChecked()
  await expect(page.getByLabel('Water interval (minutes)', { exact: true })).toHaveValue('60')
  await page.getByLabel('Water interval (minutes)', { exact: true }).fill('90')
  await page.getByLabel('Water reminders', { exact: true }).uncheck()
  await page.getByRole('button', { name: 'Save settings' }).click()
  await expect(page.getByRole('status').filter({ hasText: 'Saved on this device' })).toBeVisible()
  expect((await commandCalls(page, 'update_settings'))[0].args.settings).toMatchObject({ waterEnabled: false, waterIntervalMinutes: 90 })
  await page.reload()
  await page.getByRole('button', { name: 'Settings', exact: true }).click()
  await expect(page.getByLabel('Water reminders', { exact: true })).not.toBeChecked()
  await expect(page.getByLabel('Water interval (minutes)', { exact: true })).toHaveValue('90')
})

for (const [label, command] of [['Done', 'complete_break'], ['Snooze 5 min', 'snooze_break'], ['Skip', 'skip_break']] as const) {
  test(`water ${label} sends the current reminder UUID and hides the panel`, async ({ page }) => {
    await installDesktopFixture(page)
    await page.goto('/')
    await page.getByRole('button', { name: 'Open water reminder', exact: true }).click()
    expect((await commandCalls(page, 'start_break'))[0].args).toEqual({ category: 'water' })
    await page.goto('/?window=break')
    await expect(page.getByRole('heading', { name: 'A moment for water.' })).toBeVisible()
    await expect(page.getByText(/Consider a drink of water/)).toBeVisible()
    await page.getByRole('button', { name: label, exact: true }).focus()
    await page.keyboard.press('Enter')
    await expect(page.getByRole('heading', { name: 'No break waiting.' })).toBeVisible()
    expect((await commandCalls(page, command))[0].args).toEqual({ reminderId: '00000000-0000-0000-0000-000000000001' })
    expect(await commandCalls(page, 'hide_break_window')).toHaveLength(1)
  })
}

test('Escape hides a water reminder without completion or intake claims', async ({ page }) => {
  await installDesktopFixture(page)
  await page.goto('/')
  await page.getByRole('button', { name: 'Open water reminder', exact: true }).click()
  await page.goto('/?window=break')
  await expect(page.getByRole('heading', { name: 'A moment for water.' })).toBeVisible()
  await page.keyboard.press('Escape')
  expect(await commandCalls(page, 'complete_break')).toHaveLength(0)
  expect(await commandCalls(page, 'hide_break_window')).toHaveLength(1)
  await expect(page.getByRole('timer')).toHaveCount(0)
})
