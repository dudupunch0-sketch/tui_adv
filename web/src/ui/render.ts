import { buildPrinterFlowScene, renderPrinterFallbackText } from '../effects/printerFlow';
import { publicSecretSummary } from '../security/publicSecretGuard';
import type { GameAction, GameTurn } from '../game/types';

export function renderGameShell(turn: GameTurn): string {
  const effect = turn.encounter?.id === 'printer_prints_alone' ? renderPrinterEffectPanel() : '';
  const encounterBody = turn.encounter
    ? `<h2>${escapeHtml(turn.encounter.title)}</h2><p>${escapeHtml(turn.encounter.body)}</p>${effect}`
    : turn.ending
      ? `<h2>엔딩: ${escapeHtml(turn.ending.name)}</h2><p>${escapeHtml(turn.ending.text)}</p>${renderSecretSummary(turn)}`
      : '<p>현재 위치에서 이동할 수 있다.</p>';
  return `
<main class="fake-tui" data-app="escape-office">
  <header class="topbar">escape from the office <span>LOCAL BUILD</span></header>
  <section class="grid">
    <aside class="panel" data-panel="status">
      <h1>LOCAL STATUS</h1>
      <p>위치: ${escapeHtml(turn.location.name)}</p>
      <p>신체 반응: ${turn.state.player.health}</p>
      <p>집중도: ${turn.state.player.sanity}</p>
      <p>단말기 전원: ${turn.state.player.battery}%</p>
      <p>허기: ${turn.state.player.hunger}</p>
      <p>갈증: ${turn.state.player.thirst}</p>
    </aside>
    <section class="panel encounter" data-panel="encounter">
      <h1>CURRENT ENCOUNTER</h1>
      ${encounterBody}
      ${renderActions(turn.actions)}
    </section>
  </section>
  <footer class="panel log" data-panel="log">
    <h1>LOG</h1>
    ${renderLog(turn.state.log)}
  </footer>
</main>`.trim();
}

function renderActions(actions: GameAction[]): string {
  if (!actions.length) return '';
  return `<ol class="choices">${actions
    .map(
      (action, index) =>
        `<li><button data-action-id="${escapeHtml(action.id)}"><kbd>${index + 1}</kbd> ${escapeHtml(action.label)}</button></li>`,
    )
    .join('')}</ol>`;
}

function renderLog(log: string[]): string {
  if (!log.length) return '<p>&gt; 아직 기록 없음</p>';
  return log.slice(-5).map((entry) => `<p>&gt; ${escapeHtml(entry)}</p>`).join('');
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
