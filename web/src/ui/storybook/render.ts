import type { SceneAction, SceneBlockedAction, ScenePage, PressureCue, ResourceStatus } from '../../core/types';
import { escapeHtml } from './html';
import { renderStoryHistory } from './history';
import { renderVisualCard } from './visualCatalog';

type StoryLayout = 'visual-first' | 'text-first' | 'ending';

export function renderStorybookPage(page: ScenePage): string {
  const layout = storyLayout(page);
  return `
<main class="storybook-shell" data-app="escape-office" data-renderer="web-storybook" data-mode="${escapeHtml(page.mode)}">
  ${renderHud(page)}
  ${renderProgressRail(page)}
  <section class="storybook-page" data-story-layout="${layout}">
    ${renderStoryFlow(page, layout)}
    ${renderChoices(page.actions, page.blocked_actions, page.status_summary.turn)}
  </section>
  ${renderBottomDock(page)}
  ${renderStoryHistory(page.history_entries)}
</main>`.trim();
}

function storyLayout(page: ScenePage): StoryLayout {
  if (page.mode === 'ending') return 'ending';
  if (page.mode === 'movement') return 'visual-first';
  if (page.visual.kind.includes('location')) return 'visual-first';
  if (page.visual.id.startsWith('location:')) return 'visual-first';
  return 'text-first';
}

function renderHud(page: ScenePage): string {
  const resources = page.status_summary.resources;
  const warningRows = page.status_summary.warnings.length
    ? `<ul class="hud-warnings">${page.status_summary.warnings
        .map((warning) => `<li>${escapeHtml(warning)}</li>`)
        .join('')}</ul>`
    : '';
  const pressure = page.pressure_cues.length
    ? `<div class="storybook-pressure" data-region="pressure">${page.pressure_cues.map(renderPressureCue).join('')}</div>`
    : '';

  return `<header class="storybook-hud" data-region="status" data-danger-band="${dangerBand(page.status_summary.danger)}">
    <div class="hud-portrait" aria-label="격리 대상 초상" role="img">
      <span class="hud-portrait-noise" aria-hidden="true"></span>
      <span class="hud-portrait-badge" aria-hidden="true">ID</span>
    </div>
    <div class="hud-identity">
      <p class="hud-nameplate">${escapeHtml(page.location.name)}</p>
      <p class="hud-subtitle">${escapeHtml(page.title)}</p>
      <div class="hud-vital-slots" aria-label="핵심 상태">
        ${renderVitalSlots(resources)}
      </div>
    </div>
    <div class="hud-document" aria-label="현재 격리 문서">${escapeHtml(page.chapter_label)}</div>
    <button class="hud-settings" type="button" aria-label="설정">⚙</button>
    ${renderStatGrid(page)}
    ${warningRows}
    ${pressure}
  </header>`;
}

function renderVitalSlots(resources: ResourceStatus[]): string {
  const health = resourceById(resources, 'health');
  const sanity = resourceById(resources, 'sanity');
  return [renderSlotRow(health, 'health', '신체 반응'), renderSlotRow(sanity, 'sanity', '집중도')].join('');
}

function renderSlotRow(resource: ResourceStatus | undefined, id: string, fallbackLabel: string): string {
  const value = resource?.value ?? 0;
  const filledSlots = Math.max(0, Math.min(5, Math.ceil(value / 20)));
  const band = resource?.band ?? 'unknown';
  const label = resource?.label ?? fallbackLabel;
  const text = resource?.text ?? '측정 불가';
  const slots = Array.from({ length: 5 }, (_, index) => {
    const filled = index < filledSlots ? 'true' : 'false';
    return `<span class="hud-slot" data-filled="${filled}" aria-hidden="true"></span>`;
  }).join('');

  return `<div class="hud-slot-row" data-resource-id="${escapeHtml(id)}" data-band="${escapeHtml(
    band,
  )}" aria-label="${escapeHtml(`${label} ${text} ${value}`)}">
    <span class="hud-slot-label">${escapeHtml(label)}</span>
    <span class="hud-slot-track">${slots}</span>
  </div>`;
}

function renderStatGrid(page: ScenePage): string {
  const resourceCells = page.status_summary.resources.map(renderStatCell);
  const dangerCell = `<div data-resource-id="danger" data-band="${dangerBand(page.status_summary.danger)}">
    <dt>위험도</dt><dd><span aria-hidden="true">${statGlyph('danger')}</span>${page.status_summary.danger}</dd>
  </div>`;
  const cells = [...resourceCells, dangerCell].slice(0, 6).join('');

  return `<dl class="hud-stat-grid" aria-label="격리 진단 상태칸">${cells}</dl>`;
}

function renderStatCell(resource: ResourceStatus): string {
  return `<div data-resource-id="${escapeHtml(resource.id)}" data-band="${escapeHtml(resource.band)}">
    <dt>${escapeHtml(resource.label)}</dt>
    <dd><span aria-hidden="true">${statGlyph(resource.id)}</span>${escapeHtml(resource.text)}</dd>
  </div>`;
}

function renderProgressRail(page: ScenePage): string {
  const progress = Math.max(8, Math.min(92, page.status_summary.turn * 6));
  const styleValue = `--progress: ${progress}%`;
  return `<div class="story-progress-rail" data-danger-band="${dangerBand(
    page.status_summary.danger,
  )}" style="${styleValue}" aria-label="${escapeHtml(
    `${page.chapter_label}, 위험도 ${page.status_summary.danger}`,
  )}">
    <span class="rail-track" aria-hidden="true"></span>
    <span class="rail-fill" aria-hidden="true"></span>
    <span class="rail-marker" aria-hidden="true"></span>
    <span class="rail-knot" aria-hidden="true">✣</span>
  </div>`;
}

function renderStoryFlow(page: ScenePage, layout: StoryLayout): string {
  const visual = renderVisualCard(page.visual, page.effect_cues);
  const body = renderBody(page);
  if (layout === 'text-first') {
    return `<article class="story-flow story-flow--text-first">${body}${visual}</article>`;
  }
  if (layout === 'ending') {
    return `<article class="story-flow story-flow--ending">${visual}${body}</article>`;
  }
  return `<article class="story-flow story-flow--visual-first">${visual}${body}</article>`;
}

function renderBody(page: ScenePage): string {
  const dialogue = page.dialogue_entries.length
    ? `<section class="dialogue-stack">${page.dialogue_entries
        .map(
          (entry) => `<p data-speaker="${escapeHtml(entry.speaker)}"><strong>${escapeHtml(entry.speaker)}</strong>${escapeHtml(
            entry.text,
          )}</p>`,
        )
        .join('')}</section>`
    : '';
  const bodyBlocks = page.body_blocks
    .map(
      (block) => `<p data-body-kind="${escapeHtml(block.kind)}" data-source-id="${escapeHtml(
        block.source_id ?? '',
      )}">${escapeHtml(block.text)}</p>`,
    )
    .join('');
  const resultLog = renderInlineResultLog(page);

  return `<section class="storybook-body" data-region="body">
    <p class="eyebrow">${escapeHtml(page.mode)} · ${escapeHtml(page.location.id)}</p>
    <h1>${escapeHtml(page.title)}</h1>
    ${dialogue}
    ${bodyBlocks}
    ${resultLog}
  </section>`;
}

function renderInlineResultLog(page: ScenePage): string {
  const rows: string[] = [];
  if (page.inventory_summary.items.length) {
    rows.push(
      `+ 소지품: ${page.inventory_summary.items.map(escapeHtml).join(', ')}${
        page.inventory_summary.overflow_count ? ` 외 ${page.inventory_summary.overflow_count}개` : ''
      }`,
    );
  }
  const achievements = page.achievement_summary.newly_unlocked.length
    ? page.achievement_summary.newly_unlocked
    : page.achievement_summary.unlocked;
  if (achievements.length) rows.push(`+ 업적: ${achievements.map(escapeHtml).join(', ')}`);
  if (!rows.length) return '';

  return `<section class="story-result-log" aria-label="최근 결과">${rows
    .map((row) => `<p class="storybook-summary">${row}</p>`)
    .join('')}</section>`;
}

function renderChoices(actions: SceneAction[], blockedActions: SceneBlockedAction[], turn: number): string {
  const actionRows = actions.length
    ? actions.map(renderActionButton).join('')
    : '<li class="empty-choice">현재 실행할 수 있는 행동이 없다.</li>';
  const blockedRows = blockedActions.length
    ? `<ul class="blocked-actions">${blockedActions.map(renderBlockedAction).join('')}</ul>`
    : '';

  return `<nav class="storybook-choices" data-region="choices" aria-label="현재 선택지">
    <div class="choice-separator" aria-hidden="true"><span></span><strong>${turn}</strong><span></span></div>
    <ol>${actionRows}</ol>
    ${blockedRows}
  </nav>`;
}

function renderActionButton(action: SceneAction, index: number): string {
  const cost = action.cost_text ? `<small class="choice-cost">${escapeHtml(action.cost_text)}</small>` : '';
  return `<li><button class="choice-row" data-action-id="${escapeHtml(action.id)}" data-action-kind="${escapeHtml(
    action.kind,
  )}">
    <span class="choice-bullet" aria-hidden="true">✥</span><kbd class="choice-index">${index + 1}</kbd><span class="choice-label">${escapeHtml(
      action.label,
    )}</span>${cost}
  </button></li>`;
}

function renderBlockedAction(action: SceneBlockedAction): string {
  return `<li data-blocked-action-id="${escapeHtml(action.id)}"><span class="choice-bullet" aria-hidden="true">✧</span><span>${escapeHtml(
    action.label,
  )}</span><small>${action.reasons.map(escapeHtml).join(' · ')}</small></li>`;
}

function renderBottomDock(page: ScenePage): string {
  const inventoryCount = page.inventory_summary.items.length + page.inventory_summary.overflow_count;
  const achievementCount = page.achievement_summary.unlocked.length;
  return `<footer class="storybook-dock" aria-label="보조 메뉴">
    <a class="dock-item" href="#story-history" data-dock="log" aria-label="기록"><span aria-hidden="true">▧</span><small>기록</small></a>
    <span class="dock-item" data-dock="clues" aria-label="단서" role="img"><span aria-hidden="true">▣</span><small>단서</small></span>
    <span class="dock-item" data-dock="achievements" aria-label="업적 ${achievementCount}개" role="img"><span aria-hidden="true">◈</span><small>업적</small></span>
    <span class="dock-spacer" aria-hidden="true"></span>
    <span class="dock-item" data-dock="actions" aria-label="현재 목표" role="img"><span aria-hidden="true">✎</span><small>목표</small></span>
    <span class="dock-item" data-dock="inventory" aria-label="소지품" title="소지품 ${inventoryCount}개" role="img"><span aria-hidden="true">◒</span><small>가방</small></span>
  </footer>`;
}

function renderPressureCue(cue: PressureCue): string {
  return `<p data-pressure-kind="${escapeHtml(cue.kind)}" data-severity="${escapeHtml(cue.severity)}">${escapeHtml(
    cue.message,
  )}</p>`;
}

function resourceById(resources: ResourceStatus[], id: string): ResourceStatus | undefined {
  return resources.find((resource) => resource.id === id);
}

function dangerBand(danger: number): string {
  if (danger >= 4) return 'critical';
  if (danger >= 2) return 'warning';
  return 'low';
}

function statGlyph(resourceId: string): string {
  const glyphs: Record<string, string> = {
    health: '◆',
    sanity: '◇',
    battery: '▣',
    hunger: '▥',
    thirst: '◉',
    danger: '▲',
  };
  return glyphs[resourceId] ?? '◇';
}
