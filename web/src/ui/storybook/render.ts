import type { SceneAction, SceneBlockedAction, ScenePage, PressureCue, ResourceStatus } from '../../core/types';
import { escapeHtml } from './html';
import { renderStoryHistory } from './history';
import { renderVisualCard } from './visualCatalog';

export function renderStorybookPage(page: ScenePage): string {
  return `
<main class="storybook-shell" data-app="escape-office" data-renderer="web-storybook" data-mode="${escapeHtml(page.mode)}">
  <header class="storybook-topline">
    <span>escape from the office</span>
    <span>${escapeHtml(page.chapter_label)}</span>
    <span>${escapeHtml(page.location.name)}</span>
  </header>
  <section class="storybook-page">
    ${renderStatus(page)}
    ${renderVisualCard(page.visual, page.effect_cues)}
    ${renderBody(page)}
    ${renderChoices(page.actions, page.blocked_actions)}
    ${renderStoryHistory(page.history_entries)}
  </section>
</main>`.trim();
}

function renderStatus(page: ScenePage): string {
  const resources = page.status_summary.resources.map(renderResourceStatus).join('');
  const warnings = page.status_summary.warnings.length
    ? `<ul class="storybook-warnings">${page.status_summary.warnings
        .map((warning) => `<li>${escapeHtml(warning)}</li>`)
        .join('')}</ul>`
    : '';
  const pressure = page.pressure_cues.length
    ? `<div class="storybook-pressure" data-region="pressure">${page.pressure_cues.map(renderPressureCue).join('')}</div>`
    : '';

  return `<aside class="storybook-status" data-region="status">
    <h2>격리 진단</h2>
    <p>${escapeHtml(page.location.description)}</p>
    <dl>
      <div><dt>턴</dt><dd>${page.status_summary.turn}</dd></div>
      <div><dt>위험도</dt><dd>${page.status_summary.danger}</dd></div>
    </dl>
    <ul class="resource-strip">${resources}</ul>
    ${warnings}
    ${pressure}
  </aside>`;
}

function renderResourceStatus(resource: ResourceStatus): string {
  return `<li data-resource-id="${escapeHtml(resource.id)}" data-band="${escapeHtml(resource.band)}">
    <span>${escapeHtml(resource.label)}</span>
    <strong>${escapeHtml(resource.text)}</strong>
    <small>${resource.value}</small>
  </li>`;
}

function renderPressureCue(cue: PressureCue): string {
  return `<p data-pressure-kind="${escapeHtml(cue.kind)}" data-severity="${escapeHtml(cue.severity)}">${escapeHtml(
    cue.message,
  )}</p>`;
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
  const inventory = page.inventory_summary.items.length
    ? `<p class="storybook-summary">소지품: ${page.inventory_summary.items.map(escapeHtml).join(', ')}${
        page.inventory_summary.overflow_count ? ` 외 ${page.inventory_summary.overflow_count}개` : ''
      }</p>`
    : '';
  const achievements = page.achievement_summary.unlocked.length
    ? `<p class="storybook-summary">업적: ${page.achievement_summary.unlocked.map(escapeHtml).join(', ')}</p>`
    : '';

  return `<article class="storybook-body" data-region="body">
    <p class="eyebrow">${escapeHtml(page.mode)} · ${escapeHtml(page.location.id)}</p>
    <h1>${escapeHtml(page.title)}</h1>
    ${dialogue}
    ${bodyBlocks}
    ${inventory}
    ${achievements}
  </article>`;
}

function renderChoices(actions: SceneAction[], blockedActions: SceneBlockedAction[]): string {
  const actionRows = actions.length
    ? actions.map(renderActionButton).join('')
    : '<li class="empty-choice">현재 실행할 수 있는 행동이 없다.</li>';
  const blockedRows = blockedActions.length
    ? `<ul class="blocked-actions">${blockedActions.map(renderBlockedAction).join('')}</ul>`
    : '';

  return `<nav class="storybook-choices" data-region="choices" aria-label="현재 선택지">
    <ol>${actionRows}</ol>
    ${blockedRows}
  </nav>`;
}

function renderActionButton(action: SceneAction, index: number): string {
  const cost = action.cost_text ? `<small>${escapeHtml(action.cost_text)}</small>` : '';
  return `<li><button data-action-id="${escapeHtml(action.id)}" data-action-kind="${escapeHtml(action.kind)}">
    <kbd>${index + 1}</kbd><span>${escapeHtml(action.label)}</span>${cost}
  </button></li>`;
}

function renderBlockedAction(action: SceneBlockedAction): string {
  return `<li data-blocked-action-id="${escapeHtml(action.id)}"><span>${escapeHtml(action.label)}</span><small>${action.reasons
    .map(escapeHtml)
    .join(' · ')}</small></li>`;
}
