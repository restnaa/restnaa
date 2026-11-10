import { test as base, expect } from '@playwright/test'
import { basename, resolve } from 'node:path'
export { expect }
export const test = base.extend({
  page: async ({page},use,testInfo)=>{
    // The packaged app serves these same bundled files without TCP sockets.
    // This host intermittently returns ERR_NO_BUFFER_SPACE on loopback CSS.
    // Fulfill exact build artifacts, not alternate content, while retaining
    // missing-file and uncaught-script failures below. HTML still uses preview.
    await page.route('http://127.0.0.1:1420/assets/**',route=>route.fulfill({path:resolve('dist/assets',basename(new URL(route.request().url()).pathname))}))
    const errors:string[]=[]
    page.on('pageerror',error=>errors.push(error.message))
    page.on('requestfailed',request=>{
      const failure=request.failure()?.errorText ?? 'unknown failure'
      if(failure!=='net::ERR_ABORTED' && ['script','stylesheet'].includes(request.resourceType()))errors.push(`${request.url()}: ${failure}`)
    })
    page.on('response',response=>{if(response.status()>=400 && ['script','stylesheet'].includes(response.request().resourceType()))errors.push(`${response.status()} ${response.url()}`)})
    await use(page)
    if(errors.length)await testInfo.attach('browser-errors',{body:JSON.stringify(errors),contentType:'application/json'})
    expect(errors,'No uncaught errors or failed bundled scripts/styles').toEqual([])
  },
})
