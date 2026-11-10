import type { LocalHistory, HistoryRange, CategoryCounts } from '../lib/types'
export function historyFixture(range: HistoryRange = 'sevenDays'): LocalHistory {
  const categories: [CategoryCounts, CategoryCounts] = [
    { category: 'eye', counts: { completed: 2, skipped: 1, snoozed: 3, unanswered: 4 } },
    { category: 'movement', counts: { completed: 1, skipped: 2, snoozed: 1, unanswered: 0 } },
  ]
  return { range, fromDate: range === 'today' ? '2026-10-21' : '2026-10-15', throughDate: '2026-10-21', asOfUtc: '2026-10-21T12:00:00Z', categories,
    days: [{ date: '2026-10-21', categories }], recent: [
      { id: 'outcome-1', category: 'eye', outcome: 'snoozed', triggeredAtUtc: '2026-10-21T09:00:00Z', actedAtUtc: '2026-10-21T09:00:20Z', createdAtUtc: '2026-10-21T09:00:20Z', durationSeconds: null },
    ], totalEvents: 14, recentTruncated: false }
}
