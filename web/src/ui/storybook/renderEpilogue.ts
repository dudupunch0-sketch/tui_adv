import type { BodyBlock } from '../../core/types';
import { escapeHtml } from './html';

export function renderEpilogueBodyBlock(block: BodyBlock): string {
  const parsed = parseEpilogueBlockText(block.text);
  const heading = epilogueBlockHeading(block.kind, parsed);
  const prose = parsed.prose.length
    ? `<p class="epilogue-block-prose">${parsed.prose.map(escapeHtml).join('<br>')}</p>`
    : '';
  const metadata = parsed.metadata.length
    ? `<details class="epilogue-block-metadata">
        <summary>계약 기록</summary>
        <dl>${parsed.metadata
          .map(
            ([key, value]) =>
              `<div><dt data-field-key="${escapeHtml(key)}">${escapeHtml(epilogueMetadataLabel(key))}</dt><dd>${escapeHtml(
                value,
              )}</dd></div>`,
          )
          .join('')}</dl>
      </details>`
    : '';

  return `<section class="epilogue-block epilogue-block--${escapeHtml(
    block.kind,
  )}" data-body-kind="${escapeHtml(block.kind)}" data-source-id="${escapeHtml(block.source_id ?? '')}">
    <p class="epilogue-block-kicker">${escapeHtml(epilogueBlockKicker(block.kind))}</p>
    <h2>${escapeHtml(heading)}</h2>
    ${prose}
    ${metadata}
  </section>`;
}

function parseEpilogueBlockText(text: string): { metadata: Array<[string, string]>; prose: string[] } {
  const metadata: Array<[string, string]> = [];
  const prose: string[] = [];
  for (const rawLine of text.split('\n')) {
    const line = rawLine.trim();
    if (!line) continue;
    const match = /^([a-z_]+):\s*(.*)$/.exec(line);
    if (match) {
      metadata.push([match[1], match[2]]);
    } else {
      prose.push(line);
    }
  }
  return { metadata, prose };
}

function epilogueBlockKicker(kind: string): string {
  if (kind === 'epilogue_result') return '결산 판정';
  if (kind === 'epilogue_card') return '후일담 카드';
  if (kind === 'epilogue_suppressed') return '억제된 후보';
  if (kind === 'epilogue_contract_error') return '계약 오류';
  return '후일담';
}

function epilogueBlockHeading(kind: string, parsed: { metadata: Array<[string, string]>; prose: string[] }): string {
  const resultTitle = metadataValue(parsed, 'result_title');
  if (resultTitle) return resultTitle;
  const cardId = metadataValue(parsed, 'card_id');
  if (cardId) return cardId;
  if (parsed.prose[0]) return parsed.prose[0];
  return epilogueBlockKicker(kind);
}

function metadataValue(parsed: { metadata: Array<[string, string]> }, key: string): string | undefined {
  return parsed.metadata.find(([candidate]) => candidate === key)?.[1];
}

function epilogueMetadataLabel(key: string): string {
  const labels: Record<string, string> = {
    card_id: '카드',
    consumed_seeds: '소비한 씨앗',
    final_result_key: '결과 키',
    group: '축',
    owned_by: '소유',
    result_title: '판정명',
    routing_note: '처리 기록',
    suppressed_by: '억제 조건',
    variant: '변주',
  };
  return labels[key] ?? key;
}
