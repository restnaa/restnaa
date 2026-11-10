import type { LucideIcon } from 'lucide-react'
export function IconBadge({ icon: Icon, violet = false, small = false }: { icon: LucideIcon; violet?: boolean; small?: boolean }) {
  return <span className={`icon-badge${violet ? ' violet' : ''}${small ? ' small' : ''}`}><Icon aria-hidden="true" strokeWidth={1.8} /></span>
}
