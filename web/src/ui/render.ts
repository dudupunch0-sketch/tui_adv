import { buildPrinterFlowScene, renderPrinterFallbackText } from '../effects/printerFlow';
import { achievementById } from '../game/achievements';
import { itemsById } from '../game/content';
import { shouldDistortChoices } from '../game/state';
import { publicSecretSummary } from '../security/publicSecretGuard';
import type { GameAction, GameTurn } from '../game/types';

export function renderGameShell(turn: GameTurn): string {
  const effect = turn.encounter?.id === 'printer_prints_alone' ? renderPrinterEffectPanel() : '';
  const distortChoices = shouldDistortChoices(turn.state.player);
  const pressureWarning = distortChoices
    ? '<p class="pressure-warning">집중도가 흔들려 선택지가 부분적으로 왜곡된다.</p>'
    : '';
  const encounterBody = turn.encounter
    ? `<h2>${escapeHtml(turn.encounter.title)}</h2><p>${escapeHtml(turn.encounter.body)}</p>${effect}`
    : turn.ending
      ? `<h2>엔딩: ${escapeHtml(turn.ending.name)}</h2><p>${escapeHtml(turn.ending.text)}</p>${renderSecretSummary(turn)}`
      : '<p>현재 위치에서 이동할 수 있다.</p>';
  return `
<main class="fake-tui" data-app="escape-office">
  <header class="topbar">escape from the office <span>LOCAL BUILD</span></header>
  <section class="grid">
    <aside class="sidebar">
      <section class="panel" data-panel="status">
        <h1>LOCAL STATUS</h1>
        <p>위치: ${escapeHtml(turn.location.name)}</p>
        <p>신체 반응: ${turn.state.player.health}</p>
        <p>집중도: ${turn.state.player.sanity}</p>
        <p>단말기 전원: ${turn.state.player.battery}%</p>
        <p>허기: ${turn.state.player.hunger}</p>
        <p>갈증: ${turn.state.player.thirst}</p>
        ${pressureWarning}
      </section>
      ${renderInventoryPanel(turn)}
      ${renderAchievementsPanel(turn)}
      ${renderPressurePanel(turn)}
      ${renderControlsPanel()}
    </aside>
    <section class="panel encounter" data-panel="encounter">
      <h1>CURRENT ENCOUNTER</h1>
      ${encounterBody}
      ${renderActions(turn.actions, distortChoices)}
    </section>
  </section>
  <footer class="panel log" data-panel="log">
    <h1>LOG</h1>
    ${renderLog(turn.state.log)}
  </footer>
</main>`.trim();
}

function renderActions(actions: GameAction[], distortChoices: boolean): string {
  if (!actions.length) return '';
  return `<ol class="choices">${actions
    .map((action, index) => {
      const label = distortChoices && action.kind === 'choice' ? distortLabel(action.label) : action.label;
      return `<li><button data-action-id="${escapeHtml(action.id)}"><kbd>${index + 1}</kbd> ${escapeHtml(label)}</button></li>`;
    })
    .join('')}</ol>`;
}

function renderInventoryPanel(turn: GameTurn): string {
  const rows = turn.state.inventory.length
    ? turn.state.inventory
        .map((itemId) => {
          const item = itemsById.get(itemId);
          const useAction = turn.actions.find((action) => action.kind === 'item' && action.targetId === itemId);
          const useButton = useAction
            ? ` <button class="inline-action" data-action-id="${escapeHtml(useAction.id)}">사용</button>`
            : '';
          return `<li><span>${escapeHtml(item?.name ?? itemId)}</span>${useButton}<small>${escapeHtml(item?.description ?? '')}</small></li>`;
        })
        .join('')
    : '<li>비어 있음</li>';
  return `<section class="panel compact-panel" data-panel="inventory"><h1>INVENTORY</h1><ul class="compact-list">${rows}</ul></section>`;
}

function renderAchievementsPanel(turn: GameTurn): string {
  const rows = turn.state.unlockedAchievements.length
    ? turn.state.unlockedAchievements
        .map((achievementId) => {
          const achievement = achievementById(achievementId);
          return `<li><strong>${escapeHtml(achievement?.name ?? achievementId)}</strong><small>${escapeHtml(achievement?.description ?? '')}</small></li>`;
        })
        .join('')
    : '<li>아직 없음</li>';
  return `<section class="panel compact-panel" data-panel="achievements"><h1>ACHIEVEMENTS</h1><ul class="compact-list">${rows}</ul></section>`;
}

function renderPressurePanel(turn: GameTurn): string {
  const statuses: string[] = [];
  if (shouldDistortChoices(turn.state.player)) statuses.push('LOW SANITY: 선택지 왜곡');
  if (turn.state.player.thirst >= 60) statuses.push('HIGH THIRST: 정수기 환청');
  if (turn.state.player.hunger >= 80) statuses.push('HIGH HUNGER: 체력 압박');
  if (!statuses.length) statuses.push('압박 상태: 정상 범위');
  return `<section class="panel compact-panel" data-panel="pressure"><h1>PRESSURE</h1>${statuses
    .map((status) => `<p>${escapeHtml(status)}</p>`)
    .join('')}</section>`;
}

function renderControlsPanel(): string {
  return `<section class="panel compact-panel" data-panel="controls">
    <h1>CONTROLS / SAVE</h1>
    <p>1-9 선택 · 클릭으로 직접 실행</p>
    <p>N 새 게임</p>
    <p>localStorage 자동 저장</p>
  </section>`;
}

function renderLog(log: string[]): string {
  if (!log.length) return '<p>&gt; 아직 기록 없음</p>';
  return log.slice(-5).map((entry) => `<p>&gt; ${escapeHtml(entry)}</p>`).join('');
}

function distortLabel(label: string): string {
  if (label === '메시지를 확인한다') return '메시▒를 확▒한다';
  return Array.from(label)
    .map((character, index) => (character !== ' ' && index % 4 === 2 ? '▒' : character))
    .join('');
}

function renderPrinterEffectPanel(): string {
  const scene = buildPrinterFlowScene();
  return `<div class="anomaly-panel" data-effect="printer-flow">
    <canvas data-anomaly-canvas="printer-flow" aria-label="복합기 현실 연결 텍스트 흐름"></canvas>
    <pre>ANOMALY CANVAS READY\n${escapeHtml(renderPrinterFallbackText(scene))}</pre>
  </div>`;
}

function renderSecretSummary(turn: GameTurn): string {
  if (!turn.ending?.publicSecret) return '';
  return `<pre class="secret-summary">${escapeHtml(publicSecretSummary(turn.ending.publicSecret))}</pre>`;
}

function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}
