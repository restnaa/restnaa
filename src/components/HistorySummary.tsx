import { categories as categoryMeta } from '../lib/categories'
import type { CategoryCounts } from '../lib/types'
import { outcomeLabels } from '../lib/labels'
import { IconBadge } from './IconBadge'
export function HistorySummary({ categories }: { categories: CategoryCounts[] }) {
  return <div className="outcome-summary">{categories.map(({ category, counts }) => <div className="category-summary" key={category}>
    <IconBadge icon={categoryMeta[category].icon} small violet={category === 'movement'} />
    <div><h3>{categoryMeta[category].plural}</h3><dl className="compact-stats">{Object.entries(outcomeLabels).map(([key, label]) => <div key={key}><dt>{label}</dt><dd>{counts[key as keyof typeof counts]}</dd></div>)}</dl></div>
  </div>)}</div>
}
