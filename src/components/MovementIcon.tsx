import { createLucideIcon } from 'lucide-react'
// Bundled walking figure, matching the movement reminder's visual language.
export const MovementIcon = createLucideIcon('Movement', [
  ['circle', { cx: '13', cy: '4', r: '2', key: 'head' }],
  ['path', { d: 'm11 8-3 5-3 2m6-7 4 5 4 1M11 8l-1 7 5 6m-5-6-3 7', key: 'body' }],
])
