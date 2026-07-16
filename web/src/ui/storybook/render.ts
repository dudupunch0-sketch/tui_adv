import type {
  ActionCheckInfo,
  BodyBlock,
  CharacterSummary,
  InventoryDetail,
  ProgressionStatus,
  ResourceStatus,
  SceneContentItem,
  SceneAction,
  SceneBlockedAction,
  ScenePage,
  CheckResolution,
} from '../../core/types';
import { escapeHtml } from './html';
import { renderStoryHistory } from './history';
import {
  achievementLabel,
  hasAchievementLabel,
  inventoryItemLabel,
} from './labels';
import { renderEpilogueBodyBlock } from './renderEpilogue';
import { renderInkVisual } from './ink/renderInkVisual';

type StoryLayout = 'visual-first' | 'text-first' | 'ending';
type StoryPhase = 'story' | 'combat' | 'result' | 'collapse';

export interface StorybookRenderOptions {
  audioLabel?: string;
  motionLabel?: string;
  /** true면 판정 배너와 인라인 결과 로그를 생략한다 (행동 결과 비트가 이전 화면에서 이미 보여준 경우). */
  suppressActionResult?: boolean;
}

export function renderStorybookPage(page: ScenePage, options: StorybookRenderOptions = {}): string {
  const layout = storyLayout(page);
  const phase = storyPhase(page);
  return `
<main class="storybook-shell" data-app="tui-adv" data-renderer="web-storybook" data-game-frame="true" data-mode="${escapeHtml(
    page.mode,
  )}" data-story-phase="${phase}">
  ${renderTopBar(page)}
  <div class="game-viewport" data-region="viewport">
    <section class="storybook-page" data-story-layout="${layout}" data-story-phase="${phase}">
      ${renderStoryFlow(page, layout, options)}
      ${page.content_stream?.length ? '' : renderChoices(page)}
    </section>
  </div>
  ${renderHud(page)}
  ${renderBottomDock(page, options)}
</main>`.trim();
}

function storyLayout(page: ScenePage): StoryLayout {
  if (page.mode === 'ending') return 'ending';
  if (page.mode === 'movement') return 'visual-first';
  if (page.visual.kind.includes('location')) return 'visual-first';
  if (page.visual.id.startsWith('location:')) return 'visual-first';
  return 'text-first';
}

function storyPhase(page: ScenePage): StoryPhase {
  if (page.visual.kind === 'collapse_gate') return 'collapse';
  if (isCombatScene(page)) return 'combat';
  if (page.content_stream?.length) {
    return page.content_stream.some((item) => item.kind === 'result_summary') ? 'result' : 'story';
  }
  if (page.history_entries.length || page.achievement_summary.newly_unlocked.length) return 'result';
  return 'story';
}

function isCombatScene(page: ScenePage): boolean {
  return (
    page.visual.kind === 'combat_intervention' ||
    page.visual.id.includes('brawl') ||
    page.visual.id.includes('fight') ||
    page.visual.id.includes('combat')
  );
}

function renderTopBar(page: ScenePage): string {
  return `<header class="game-topbar" data-region="topbar">
    <span class="game-topbar__location" aria-label="현재 장소 ${escapeHtml(page.location.name)}"><span class="game-topbar__location-glyph" aria-hidden="true">地</span>${escapeHtml(page.location.name)}</span>
    <p class="hud-document" aria-label="현재 기록 ${escapeHtml(documentLabel(page))} · ${page.status_summary.turn}턴">${escapeHtml(documentLabel(page))}<span class="game-topbar__turn"> · ${page.status_summary.turn}턴</span></p>
  </header>`;
}

function renderHud(page: ScenePage): string {
  const resources = storyResources(page.status_summary.resources);
  return `<header class="storybook-hud" data-region="status" data-danger-band="${dangerBand(page.status_summary.danger)}">
    <div class="hud-vital-slots" aria-label="핵심 상태">${renderVitalSlots(resources)}</div>
    <div class="hud-center">
      ${renderProgressionGauge(page.progression, 'hud')}
    </div>
    ${renderProgressRail(page)}
    <button type="button" class="hud-drawer-toggle" data-player-action="toggle-storybook-drawer" aria-expanded="false" aria-controls="storybook-info-drawer"><span class="hud-drawer-toggle__glyph" aria-hidden="true">詳</span><span class="hud-drawer-toggle__label">상세</span></button>
  </header>`;
}

function renderVitalSlots(resources: ResourceStatus[]): string {
  const health = resourceById(resources, 'health');
  const sanity = resourceById(resources, 'sanity');
  return [renderVitalRow(health, 'health', '체력'), renderVitalRow(sanity, 'sanity', '정신력')].join('');
}

function renderVitalRow(resource: ResourceStatus | undefined, id: string, fallbackLabel: string): string {
  const value = resource?.value ?? 0;
  const fill = Math.max(0, Math.min(100, value));
  const band = resource?.band ?? 'unknown';
  const label = resource?.label ?? fallbackLabel;
  const text = resource?.text ?? '측정 불가';

  return `<div class="hud-vital" data-resource-id="${escapeHtml(id)}" data-band="${escapeHtml(
    band,
  )}" aria-label="${escapeHtml(`${label} ${text} ${value}`)}">
    <span class="hud-vital__label">${escapeHtml(label)}</span>
    <span class="hud-vital__track" aria-hidden="true"><span class="hud-vital__fill" style="--fill: ${fill}%"></span></span>
    <span class="hud-vital__value">${escapeHtml(String(value))}</span>
  </div>`;
}

function renderProgressionGauge(progression: ProgressionStatus | undefined, variant: 'hud' | 'drawer'): string {
  if (!progression || progression.target <= 0) return '';
  const percent = Math.max(0, Math.min(100, (progression.experience / progression.target) * 100));
  const readout = `${progression.label} ${progression.experience} / ${progression.target}`;
  if (variant === 'hud') {
    return `<span class="hud-progression" role="img" aria-label="${escapeHtml(readout)}" title="${escapeHtml(
      readout,
    )}" style="--fill: ${percent}%"><span class="hud-progression__label" aria-hidden="true">${escapeHtml(
      progression.label,
    )}</span><span class="hud-progression__track" aria-hidden="true"><span class="hud-progression__fill"></span></span></span>`;
  }
  return `<div class="drawer-progression" role="img" aria-label="${escapeHtml(readout)}" style="--fill: ${percent}%">
    <span class="drawer-progression__label">${escapeHtml(progression.label)}</span>
    <span class="drawer-progression__track" aria-hidden="true"><span class="drawer-progression__fill"></span></span>
    <span class="drawer-progression__value">${progression.experience} / ${progression.target}</span>
  </div>`;
}

function renderProgressRail(page: ScenePage): string {
  const progress = Math.max(10, Math.min(90, page.status_summary.danger * 14 + 16));
  const styleValue = `--progress: ${progress}%`;
  return `<div class="story-progress-rail" data-danger-band="${dangerBand(
    page.status_summary.danger,
  )}" style="${styleValue}" aria-label="${escapeHtml(`위험도 ${page.status_summary.danger}`)}">
    <span class="rail-track" aria-hidden="true"></span>
    <span class="rail-fill" aria-hidden="true"></span>
    <span class="rail-marker" aria-hidden="true"></span>
    <span class="rail-knot" aria-hidden="true">✣</span>
  </div>`;
}

function renderStoryFlow(page: ScenePage, layout: StoryLayout, options: StorybookRenderOptions = {}): string {
  if (page.content_stream?.length) return renderOrderedStoryFlow(page, options);
  const visual = renderInkVisual(page.visual, page.effect_cues, page.mode);
  const body = renderBody(page, options);
  if (layout === 'text-first') {
    return `<article class="story-flow story-flow--text-first">${body}${visual}</article>`;
  }
  if (layout === 'ending') {
    return `<article class="story-flow story-flow--ending">${visual}${body}</article>`;
  }
  return `<article class="story-flow story-flow--visual-first">${visual}${body}</article>`;
}

function renderOrderedStoryFlow(page: ScenePage, options: StorybookRenderOptions = {}): string {
  const title = page.title === page.location.name ? '' : `<h1>${escapeHtml(page.title)}</h1>`;
  const pressureNotes = [...page.status_summary.warnings, ...page.pressure_cues.map((cue) => cue.message)];
  const items = page.content_stream!.map((item) => renderContentItem(item, page)).join('');
  const hasResultItem = page.content_stream!.some((item) => item.kind === 'result_summary');

  return `<article class="story-flow story-flow--ordered" data-region="story-flow">
    <header class="story-flow__heading">
      <p class="storybook-location">${escapeHtml(page.location.name)}</p>
      ${title}
      ${pressureNotes.length ? `<aside class="storybook-pressure" data-region="pressure">${pressureNotes.map((note) => `<p>${escapeHtml(note)}</p>`).join('')}</aside>` : ''}
    </header>
    <section class="storybook-body storybook-body--ordered" data-region="body">${items}</section>
    ${options.suppressActionResult ? '' : renderCheckResolution(page)}
    ${hasResultItem || options.suppressActionResult ? '' : renderInlineResultLog(page)}
  </article>`;
}

function renderContentItem(item: SceneContentItem, page: ScenePage): string {
  const stageId = escapeHtml(item.stage_id ?? '');
  const attrs = `data-content-kind="${escapeHtml(item.kind)}" data-stage-id="${stageId}"`;
  if (item.kind === 'illustration') {
    if (item.placeholder || !item.visual_id) {
      const label = item.alt?.trim() || (item.visual_id ? `${item.visual_id}.png` : 'no image');
      return `<figure class="story-illustration story-illustration--placeholder" ${attrs} data-region="visual" data-placeholder="true"><div aria-label="${escapeHtml(label)}"><span>NO IMAGE</span><small>${escapeHtml(label)}</small></div></figure>`;
    }
    return `<div class="story-illustration" ${attrs}>${renderInkVisual(
      { id: item.visual_id, kind: 'illustration', alt: item.alt ?? item.visual_id, source_id: item.stage_id ?? null },
      page.effect_cues,
      page.mode,
    )}</div>`;
  }
  if (item.kind === 'choice' || item.kind === 'continue') {
    const actions = item.actions?.length ? item.actions : page.actions.slice(0, 1);
    const isEmptyChoice = item.kind === 'choice' && !item.actions?.length && page.mode !== 'ending';
    return renderChoices(
      page,
      actions,
      item.stage_id ?? undefined,
      item.kind === 'choice' || isEmptyChoice,
    );
  }
  if (item.kind === 'dialogue') {
    const speaker = item.speaker?.trim();
    return `<section class="story-content-block dialogue-stack" ${attrs}>${item.text ? `<p${speaker ? ` data-speaker="${escapeHtml(speaker)}"` : ''}>${speaker ? `<strong>${escapeHtml(speaker)}</strong>` : ''}${escapeHtml(item.text)}</p>` : ''}</section>`;
  }
  const text = item.text ?? '';
  if (item.kind.startsWith('epilogue_')) {
    return `<div class="story-content-block" ${attrs}>${renderEpilogueBodyBlock({
      kind: item.kind,
      text,
      source_id: item.stage_id ?? null,
    })}</div>`;
  }
  const tag = item.kind === 'document' || item.kind === 'cheongirok' ? 'blockquote' : 'section';
  return `<${tag} class="story-content-block story-content-block--${escapeHtml(item.kind)}" ${attrs}>${text
    .split('\n')
    .filter(Boolean)
    .map((line) => `<p>${escapeHtml(line)}</p>`)
    .join('')}</${tag}>`;
}

function renderBody(page: ScenePage, options: StorybookRenderOptions = {}): string {
  const title = page.title === page.location.name ? '' : `<h1>${escapeHtml(page.title)}</h1>`;
  const dialogueTexts = new Set(page.dialogue_entries.map((entry) => entry.text.trim()));
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
    .filter((block) => !dialogueTexts.has(block.text.trim()))
    .map(renderBodyBlock)
    .join('');
  const resultLog = options.suppressActionResult ? '' : renderInlineResultLog(page);

  const pressureNotes = [...page.status_summary.warnings, ...page.pressure_cues.map((cue) => cue.message)];
  const checkResolution = options.suppressActionResult ? '' : renderCheckResolution(page);
  return `<section class="storybook-body" data-region="body">
    ${title}
    ${pressureNotes.length ? `<aside class="storybook-pressure" data-region="pressure">${pressureNotes.map((note) => `<p>${escapeHtml(note)}</p>`).join('')}</aside>` : ''}
    ${dialogue}
    ${bodyBlocks}
    ${checkResolution}
    ${resultLog}
  </section>`;
}

/**
 * 행동 결과 비트: 선택지를 누른 "이전 화면"에서 판정 배너와 결과 로그를 먼저 보여주기 위한 조각.
 * content_stream에 작가가 배치한 result_summary 스테이지가 있으면 (그건 다음 화면의 서사이므로)
 * 판정 배너만 포함한다. 보여줄 것이 없으면 빈 문자열.
 */
export function renderActionResultBeat(page: ScenePage): string {
  const check = renderCheckResolution(page);
  const hasStreamResult = page.content_stream?.some((item) => item.kind === 'result_summary') ?? false;
  const log = hasStreamResult ? '' : renderInlineResultLog(page);
  if (!check && !log) return '';
  return `<section class="action-result-beat" data-region="action-result" aria-live="polite">
    ${check}
    ${log}
    <p class="action-result-beat__continue"><span aria-hidden="true">✥</span>화면을 누르면 계속</p>
  </section>`;
}

function renderCheckResolution(page: ScenePage): string {
  const check: CheckResolution | undefined = page.check_result;
  if (!check) return '';

  const verdict = check.success ? '성공' : '실패';
  const outcome = check.success ? 'success' : 'failure';

  const diceGlyphs = ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'];
  const d1 = diceGlyphs[check.dice[0] - 1] ?? '⚀';
  const d2 = diceGlyphs[check.dice[1] - 1] ?? '⚀';

  const insightBonus = check.insight_bonus ?? 0;
  const insightText = insightBonus > 0 ? ` +기연 ${insightBonus}` : '';
  const mathText = `2d6 ${check.dice[0]}+${check.dice[1]} +${escapeHtml(check.ability_label)} ${check.ability_value}${insightText} = ${check.total} / 목표 ${check.difficulty}`;

  const sealGlyph = check.success ? '成' : '敗';
  return `<aside class="check-resolution" data-region="check-result" data-check-outcome="${outcome}" data-ability-id="${escapeHtml(check.ability_id)}" aria-label="판정 결과: ${verdict}">
  <span class="check-resolution__dice" aria-hidden="true"><i class="check-die">${d1}</i><i class="check-die">${d2}</i></span>
  <span class="check-resolution__math">${mathText}</span>
  <span class="check-resolution__verdict"><span class="check-resolution__seal" aria-hidden="true">${sealGlyph}</span>${verdict}</span>
</aside>`;
}

function renderBodyBlock(block: BodyBlock): string {
  if (
    block.kind === 'epilogue_result' ||
    block.kind === 'epilogue_card' ||
    block.kind === 'epilogue_suppressed' ||
    block.kind === 'epilogue_contract_error'
  ) {
    return renderEpilogueBodyBlock(block);
  }
  return `<p data-body-kind="${escapeHtml(block.kind)}" data-source-id="${escapeHtml(
    block.source_id ?? '',
  )}">${escapeHtml(block.text)}</p>`;
}

function renderInlineResultLog(page: ScenePage): string {
  const rows: string[] = [];
  const latestResult = page.history_entries[page.history_entries.length - 1];
  const hasFinalEpilogueBlocks = page.body_blocks.some((block) => block.kind.startsWith('epilogue_'));
  if (latestResult && !hasFinalEpilogueBlocks) {
    rows.push(...latestResult.text.split('\n'));
  }
  if (page.inventory_summary.items.length) {
    rows.push(`+ 소지품 ${page.inventory_summary.items.length + page.inventory_summary.overflow_count}개`);
  }
  const achievements = page.achievement_summary.newly_unlocked.length
    ? page.achievement_summary.newly_unlocked
    : page.achievement_summary.unlocked;
  if (achievements.length) {
    rows.push(`+ 업적: ${achievements.map(id => renderAchievementLabel(id, page)).join(', ')}`);
  }
  if (!rows.length) return '';

  return `<section class="story-result-log" aria-label="최근 결과">${rows
    .map(renderResultLogLine)
    .join('')}</section>`;
}

function renderResultLogLine(line: string): string {
  if (line.startsWith('+ ')) {
    return `<p class="storybook-summary result-gain">${line}</p>`;
  }
  if (line.startsWith('- ')) {
    return `<p class="storybook-summary result-loss">${line}</p>`;
  }
  return `<p class="storybook-summary">${line}</p>`;
}

function renderAchievementLabel(id: string, page: ScenePage): string {
  const translationNote = hasAchievementLabel(id, page) ? '' : '<small class="storybook-translation-note">미번역</small>';
  return `${escapeHtml(achievementLabel(id, page))}${translationNote}`;
}

function renderChoices(
  page: ScenePage,
  actions: SceneAction[] = page.actions,
  stageId?: string,
  includeBlockedActions = true,
): string {
  const actionRows = actions.length
    ? renderActionRows(actions)
    : renderEmptyChoiceRows(page.mode === 'ending');
  const blockedRows = includeBlockedActions && page.blocked_actions.length > 0
    ? `<ul class="blocked-actions">${page.blocked_actions.map(renderBlockedAction).join('')}</ul>`
    : '';

  return `<nav class="storybook-choices" data-region="choices"${stageId ? ` data-stage-id="${escapeHtml(stageId)}"` : ''} aria-label="현재 선택지">
    <div class="choice-separator ink-rule" aria-hidden="true"><span></span><i>✣</i><span></span></div>
    <ol>${actionRows}</ol>
    ${blockedRows}
  </nav>`;
}

function renderActionRows(actions: SceneAction[]): string {
  const showMoveGroupLabel =
    actions.filter((action) => action.kind === 'move').length >= 2 && actions.some((action) => action.kind === 'choice');
  let moveGroupLabelRendered = false;

  return actions
    .map((action, index) => {
      const moveGroupLabel =
        showMoveGroupLabel && action.kind === 'move' && !moveGroupLabelRendered
          ? '<li class="choice-group-label" aria-hidden="true">이동</li>'
          : '';
      if (moveGroupLabel) moveGroupLabelRendered = true;
      return `${moveGroupLabel}${renderActionButton(action, index)}`;
    })
    .join('');
}

function renderEmptyChoiceRows(isEnding: boolean): string {
  const message = isEnding ? '기록의 이 장은 여기서 끝났다.' : '현재 실행할 수 있는 행동이 없다.';
  return `<li class="empty-choice">${message}</li>
    <li><button type="button" class="choice-row" data-player-action="show-start">
      <span class="choice-bullet" aria-hidden="true">✥</span>
      <span class="choice-label">처음 화면으로 돌아간다</span>
    </button></li>`;
}

function checkBand(percent: number): string {
  if (percent >= 70) return 'favorable';
  if (percent >= 40) return 'uncertain';
  return 'risky';
}

function renderCheckBadge(check: ActionCheckInfo | undefined): string {
  if (!check) return '';
  const percent = Math.max(0, Math.min(100, check.success_percent));
  return `<span class="choice-check" data-ability-id="${escapeHtml(check.ability_id)}" data-check-band="${checkBand(
    check.success_percent,
  )}"><span class="choice-check__ability">${escapeHtml(
    check.ability_label,
  )} 판정</span><span class="choice-check__gauge" style="--odds: ${percent}%" aria-hidden="true"></span><span class="choice-check__odds">성공 ${check.success_percent.toFixed(1)}%</span></span>`;
}

function renderActionButton(action: SceneAction, index: number): string {
  const bullet = action.kind === 'move' ? '➤' : action.kind === 'use' ? '◈' : '✥';
  const cost = action.cost_text ? `<small class="choice-cost">${escapeHtml(action.cost_text)}</small>` : '';
  const check = renderCheckBadge(action.check);
  return `<li><button class="choice-row" data-action-id="${escapeHtml(action.id)}" data-action-kind="${escapeHtml(
    action.kind,
  )}">
    <span class="choice-bullet" data-bullet-kind="${escapeHtml(action.kind)}" aria-hidden="true">${bullet}</span><kbd class="choice-index">${index + 1}</kbd><span class="choice-label">${escapeHtml(
      action.label,
    )}</span>${cost}${check}
  </button></li>`;
}

function renderBlockedAction(action: SceneBlockedAction): string {
  const cost = action.cost_text ? `<small class="choice-cost">${escapeHtml(action.cost_text)}</small>` : '';
  const check = renderCheckBadge(action.check);
  return `<li data-blocked-action-id="${escapeHtml(action.id)}"><span class="choice-bullet" aria-hidden="true">✧</span><span>${escapeHtml(
    action.label,
  )}</span>${cost}${check}<small>${action.reasons.map(escapeHtml).join(' · ')}</small></li>`;
}

function renderCharacterSummary(summary: CharacterSummary | undefined): string {
  if (!summary) {
    return '';
  }
  const titlePart = summary.title_label
    ? summary.title_description
      ? `<button type="button" class="character-title-seal" data-disclosure-toggle data-disclosure-target="trait-description" aria-expanded="false">${escapeHtml(summary.title_label)}</button> `
      : `<span class="character-title-seal">${escapeHtml(summary.title_label)}</span> `
    : '';
  const nameLine = `<div class="character-name-line" data-region="character">${titlePart}<span class="character-name">${escapeHtml(
    summary.name,
  )}</span></div>`;
  const statPoints = summary.stat_points ?? 0;
  const abilitiesRows = summary.abilities
    .map((ability) => {
      const train = statPoints > 0 && ability.value < 5
        ? `<button type="button" class="ability-train" data-action-id="train:${escapeHtml(ability.id)}" aria-label="${escapeHtml(ability.label)} 수련">+</button>`
        : '';
      return `<li class="ability-row" data-ability-id="${escapeHtml(ability.id)}"><strong>${escapeHtml(ability.label)}</strong> <span class="ability-value">${ability.value}</span>${train}</li>`;
    })
    .join('');
  const pointsBadge = statPoints > 0
    ? `<span class="ability-points" data-region="ability-points">수련 가능 ${statPoints}</span>`
    : '';
  const traitDescription = summary.title_description
    ? `<div id="trait-description" class="disclosure-panel trait-description" hidden><p>${escapeHtml(summary.title_description)}</p><small>효과: 아직 새겨지지 않음</small></div>`
    : '';
  return `<section aria-label="인물" class="character-summary-section">
    <h2><span aria-hidden="true">人</span>인물${pointsBadge}</h2>
    ${nameLine}
    ${traitDescription}
    <ul class="ability-grid">${abilitiesRows}</ul>
  </section>`;
}

function renderBottomDock(page: ScenePage, options: StorybookRenderOptions): string {
  const resources = [
    ...storyResources(page.status_summary.resources),
    { id: 'danger', label: '위험', text: dangerBandLabel(page.status_summary.danger), value: page.status_summary.danger },
  ];
  const statusRows = resources
    .map((resource) => `<li title="${escapeHtml(String(resource.value))}"><strong>${escapeHtml(resource.label)}</strong>${escapeHtml(resource.text)}</li>`)
    .join('');
  return `<details class="storybook-dock" id="storybook-info-drawer" aria-label="정보 드로어">
    <summary aria-label="정보 열기"><span aria-hidden="true">✦</span><span>기록과 소지품</span></summary>
    <div class="dock-sheet">
      <header class="dock-sheet__head">
        <span class="dock-sheet__title"><span aria-hidden="true">記</span>상세 기록</span>
        <button type="button" class="dock-sheet__close" data-player-action="toggle-storybook-drawer" aria-label="상세 닫기"><span aria-hidden="true">✕</span></button>
      </header>
      <section aria-label="현재 상태"><h2><span aria-hidden="true">狀</span>상태</h2><ul>${statusRows}</ul>${renderProgressionGauge(
        page.progression,
        'drawer',
      )}</section>
      ${renderCharacterSummary(page.character_summary)}
      ${renderInsightDrawer(page)}
      ${renderInventoryDrawer(page)}
      ${renderAchievementDrawer(page)}
      <section aria-label="기록"><h2><span aria-hidden="true">冊</span>기록</h2>${renderStoryHistory(page.history_entries)}</section>
      ${renderDrawerMenu(options)}
    </div>
  </details>`;
}

function renderInventoryDrawer(page: ScenePage): string {
  const detailsById = new Map((page.inventory_details ?? []).map((detail) => [detail.id, detail]));
  const items = page.inventory_summary.items.length
    ? `<ul>${page.inventory_summary.items
        .map((id, index) => renderInventoryItem(id, detailsById.get(id), page, index))
        .join('')}${
        page.inventory_summary.overflow_count > 0
          ? `<li class="dock-drawer-overflow">…외 ${page.inventory_summary.overflow_count}개</li>`
          : ''
      }</ul>`
    : '<p>아직 지닌 것이 없다.</p>';
  return `<section aria-label="소지품" data-dock="inventory"><h2><span aria-hidden="true">囊</span>소지품</h2>${items}</section>`;
}

function renderInventoryItem(id: string, detail: InventoryDetail | undefined, page: ScenePage, index: number): string {
  const itemId = escapeHtml(id);
  const label = detail?.name ?? inventoryItemLabel(id, page);
  const icon = `<span class="item-icon" data-item-icon="${itemId}" style="--icon-hue: ${itemIconHue(id)}" aria-hidden="true"></span>`;
  if (!detail) {
    return `<li><div class="item-row item-row--static" data-item-id="${itemId}">${icon}<span>${escapeHtml(label)}</span></div></li>`;
  }

  const targetId = `item-detail-${index}`;
  const useAction = page.actions.find((action) => action.id === `use:${id}`);
  const detailMarkup = `<div id="${targetId}" class="disclosure-panel item-detail" hidden><p>${escapeHtml(detail.description)}</p><small>${escapeHtml(detail.item_type || '아이템')}</small>${detail.usable ? useAction ? `<button type="button" class="item-use" data-action-id="${escapeHtml(useAction.id)}">사용</button>` : '<button type="button" class="item-use" disabled>지금은 쓸 수 없다</button>' : ''}</div>`;
  return `<li><button type="button" class="item-row" data-item-id="${itemId}" data-disclosure-toggle data-disclosure-target="${targetId}" aria-expanded="false">${icon}<span>${escapeHtml(label)}</span></button>${detailMarkup}</li>`;
}

function itemIconHue(id: string): number {
  let hash = 0;
  for (const char of id) hash = (hash * 31 + char.charCodeAt(0)) % 360;
  return hash;
}

function renderInsightDrawer(page: ScenePage): string {
  const insights = page.insights ?? [];
  const items = insights.length
    ? `<ul>${insights.map((insight, index) => {
        const targetId = `insight-detail-${index}`;
        const effect = insight.effect_text ? `<small class="insight-effect">${escapeHtml(insight.effect_text)}</small>` : '';
        return `<li><button type="button" class="insight-row" data-disclosure-toggle data-disclosure-target="${targetId}" aria-expanded="false"><span>${escapeHtml(insight.name)}</span>${effect}</button><div id="${targetId}" class="disclosure-panel insight-detail" hidden><p>${escapeHtml(insight.description)}</p></div></li>`;
      }).join('')}</ul>`
    : '<p>아직 맺은 기연이 없다.</p>';
  return `<section aria-label="기연" data-dock="insights"><h2><span aria-hidden="true">緣</span>기연</h2>${items}</section>`;
}

function renderAchievementDrawer(page: ScenePage): string {
  const newlyUnlocked = new Set(page.achievement_summary.newly_unlocked);
  const items = page.achievement_summary.unlocked.length
    ? `<ul>${page.achievement_summary.unlocked
        .map((id) => {
          const newlyMarked = newlyUnlocked.has(id)
            ? '<span class="dock-new-mark" aria-hidden="true"></span><span class="sr-only">새로 새김</span>'
            : '';
          return `<li>${newlyMarked}${renderDrawerLabel(id, achievementLabel, hasAchievementLabel, page)}</li>`;
        })
        .join('')}</ul>`
    : '<p>아직 새긴 업적이 없다.</p>';
  return `<section aria-label="업적" data-dock="achievements"><h2><span aria-hidden="true">勳</span>업적</h2>${items}</section>`;
}

function renderDrawerMenu(options: StorybookRenderOptions): string {
  const audioLabel = options.audioLabel ?? '소리';
  const motionLabel = options.motionLabel ?? '연출';
  return `<section class="dock-menu" role="menu" aria-label="게임 메뉴"><h2><span aria-hidden="true">器</span>메뉴</h2>
    <button type="button" data-player-action="show-start" role="menuitem">처음 화면</button>
    <button type="button" data-player-action="abandon-run" role="menuitem">포기하기</button>
    <button type="button" data-player-action="toggle-audio" role="menuitem">${escapeHtml(audioLabel)}</button>
    <button type="button" data-player-action="cycle-motion" role="menuitem">${escapeHtml(motionLabel)}</button>
  </section>`;
}

function renderDrawerLabel(
  id: string,
  labelForId: (id: string, page?: ScenePage) => string,
  hasLabel: (id: string, page?: ScenePage) => boolean,
  page: ScenePage,
): string {
  const translationNote = hasLabel(id, page) ? '' : '<small class="storybook-translation-note">미번역</small>';
  return `${escapeHtml(labelForId(id, page))}${translationNote}`;
}

function resourceById(resources: ResourceStatus[], id: string): ResourceStatus | undefined {
  return resources.find((resource) => resource.id === id);
}

function storyResources(resources: ResourceStatus[]): ResourceStatus[] {
  return ['health', 'sanity']
    .map((id) => resourceById(resources, id))
    .filter((resource): resource is ResourceStatus => resource !== undefined)
    .map((resource) => ({
      ...resource,
      label: resource.id === 'health' ? '체력' : resource.id === 'sanity' ? '정신력' : resource.label,
    }));
}

function documentLabel(page: ScenePage): string {
  const rawLabel = page.chapter_label.trim();
  if (rawLabel.toLowerCase().includes('storypack')) return '기록';
  if (/격리\s*\d+\s*턴/.test(rawLabel)) return page.mode === 'ending' ? '결말' : '기록';
  return rawLabel || '기록';
}

function dangerBandLabel(danger: number): string {
  const band = dangerBand(danger);
  return band === 'critical' ? '위급' : band === 'warning' ? '주의' : '낮음';
}

function dangerBand(danger: number): string {
  if (danger >= 4) return 'critical';
  if (danger >= 2) return 'warning';
  return 'low';
}
