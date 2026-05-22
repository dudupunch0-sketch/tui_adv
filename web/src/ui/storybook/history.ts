import type { HistoryEntry } from '../../core/types';
import { escapeHtml } from './html';

export function renderStoryHistory(entries: HistoryEntry[]): string {
  const rows = entries.length
    ? entries
        .slice(-5)
        .map(
          (entry) =>
            `<li data-history-kind="${escapeHtml(entry.kind)}"><span>${escapeHtml(entry.kind)}</span>${escapeHtml(entry.text)}</li>`,
        )
        .join('')
    : '<li data-history-kind="empty"><span>system</span>아직 기록 없음</li>';

  return `<aside class="storybook-history" data-region="history"><h2>최근 기록</h2><ol>${rows}</ol></aside>`;
}
