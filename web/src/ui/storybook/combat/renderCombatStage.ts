import type {
  CombatCombatantReport,
  CombatConclusionOutcome,
  CombatConclusionReason,
  CombatConclusionReport,
  CombatSpectatorCue,
  CombatSpectatorLogEntry,
  CombatSpectatorPage,
  CombatSpectatorPiece,
  CombatSpectatorView,
  CombatSide,
} from '../../../core/types';
import { escapeHtml } from '../html';
import {
  combatLogTemplateLine,
  isKnownCombatLogTemplateId,
  roundHundredthsToInt,
} from './combatLogTemplates';

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-2 — 관전 표면 렌더러.
//
// Hard invariant: this module only *formats* `page.combat`. It never
// recomputes damage, hit/miss, win/loss, cue derivation, log-importance
// filtering, damage totals, or "most damage" selection — escape-core has
// already decided all of that (정본 13). The only math performed here is
// display conversion: hundredths -> rounded int, tick -> elapsed ms,
// coordinate -> board percentage projection, array length display.
// ---------------------------------------------------------------------------

/** cue 5종 -> web 글리프 (I11 cue 표. terminal의 `>`/`<`/`~`/`!`/`x` 표식과
 * 짝을 이룬다 — 한쪽만 고치지 말 것, `crates/escape-terminal/src/snapshot.rs`
 * 의 `combat_cue_symbol` 참고). */
const CUE_GLYPHS: Record<CombatSpectatorCue, string> = {
  attack: '攻',
  hit: '打',
  evade: '避',
  balance_broken: '傾',
  incapacitated: '倒',
};

/** cue 5종 -> 접근성 대체 표에 쓰는 한국어 라벨. */
const CUE_LABELS: Record<CombatSpectatorCue, string> = {
  attack: '공격',
  hit: '피격',
  evade: '회피',
  balance_broken: '균형 붕괴',
  incapacitated: '전투불능',
};

/** 진영 글리프. 색만으로 진영을 구분하지 않기 위한 텍스트 대체 (I9). */
const SIDE_GLYPHS: Record<CombatSide, string> = {
  ally: '我',
  enemy: '敵',
};

const SIDE_LABELS: Record<CombatSide, string> = {
  ally: '아군',
  enemy: '적군',
};

/** `view.frames`의 마지막 프레임만 그린다 (정적 스냅샷 — 1d-3이 시간축을
 * 얹는다). 좌표는 마지막 프레임 말들의 min/max로 0~100% 비례 투영한다.
 * `span === 0`이면 0으로 나누지 않고 50%(중앙)에 둔다. */
export function renderCombatBoard(view: CombatSpectatorView): string {
  const frame = view.frames.length ? view.frames[view.frames.length - 1] : undefined;

  if (!frame) {
    return `<div class="combat-stage__board" data-region="combat-board" role="img" aria-label="전투 판, 표시할 프레임이 없다">
      <p class="combat-board__empty">표시할 프레임이 없다.</p>
    </div>
    <table class="combat-board__table sr-only"><caption>전투 판 요약 표</caption><thead><tr><th scope="col">말 id</th><th scope="col">진영</th><th scope="col">좌표</th><th scope="col">상태</th><th scope="col">cue</th></tr></thead><tbody></tbody></table>`;
  }

  if (!frame.pieces.length) {
    return `<div class="combat-stage__board" data-region="combat-board" role="img" aria-label="전투 판, tick ${frame.tick}, 표시할 말이 없다">
      <p class="combat-board__empty">표시할 말이 없다 (전투원 0명).</p>
    </div>
    <table class="combat-board__table sr-only"><caption>전투 판 요약 표</caption><thead><tr><th scope="col">말 id</th><th scope="col">진영</th><th scope="col">좌표</th><th scope="col">상태</th><th scope="col">cue</th></tr></thead><tbody></tbody></table>`;
  }

  const pieces = frame.pieces;
  const xs = pieces.map((p) => p.position.x);
  const ys = pieces.map((p) => p.position.y);
  const minX = Math.min(...xs);
  const maxX = Math.max(...xs);
  const minY = Math.min(...ys);
  const maxY = Math.max(...ys);

  const pieceMarkup = pieces
    .map((p) => renderPiece(p, projectAxis(p.position.x, minX, maxX), projectAxis(p.position.y, minY, maxY)))
    .join('');

  const allyCount = pieces.filter((p) => p.side === 'ally').length;
  const enemyCount = pieces.filter((p) => p.side === 'enemy').length;
  const boardLabel = `전투 판, tick ${frame.tick}, 아군 ${allyCount}명 · 적군 ${enemyCount}명`;

  return `<div class="combat-stage__board" data-region="combat-board" role="img" aria-label="${escapeHtml(boardLabel)}">
    ${pieceMarkup}
  </div>
  ${renderBoardTable(pieces)}`;
}

function renderPiece(piece: CombatSpectatorPiece, xPercent: number, yPercent: number): string {
  const cueAttrs = piece.cues.map((cue) => ` data-cue-${escapeHtml(cue)}="true"`).join('');
  const cueSpans = piece.cues
    .map(
      (cue) =>
        `<span class="combat-board__cue" data-cue="${escapeHtml(cue)}" aria-hidden="true">${CUE_GLYPHS[cue]}</span>`,
    )
    .join('');
  return `<div class="combat-board__piece" data-piece-id="${escapeHtml(piece.id)}" data-side="${escapeHtml(
    piece.side,
  )}" data-active="${piece.active}"${cueAttrs} style="--piece-x: ${formatPercent(xPercent)}%; --piece-y: ${formatPercent(
    yPercent,
  )}%">
    <span class="combat-board__glyph" aria-hidden="true">${SIDE_GLYPHS[piece.side]}</span>${cueSpans}
  </div>`;
}

function renderBoardTable(pieces: CombatSpectatorPiece[]): string {
  const rows = pieces
    .map((p) => {
      const status = p.active ? '생존' : '비활성';
      const cueText = p.cues.length ? p.cues.map((cue) => CUE_LABELS[cue]).join(', ') : '없음';
      return `<tr><td>${escapeHtml(p.id)}</td><td>${escapeHtml(SIDE_LABELS[p.side])}</td><td>(${String(
        p.position.x,
      )}, ${String(p.position.y)})</td><td>${escapeHtml(status)}</td><td>${escapeHtml(cueText)}</td></tr>`;
    })
    .join('');
  return `<table class="combat-board__table sr-only"><caption>전투 판 요약 표</caption><thead><tr><th scope="col">말 id</th><th scope="col">진영</th><th scope="col">좌표</th><th scope="col">상태</th><th scope="col">cue</th></tr></thead><tbody>${rows}</tbody></table>`;
}

/** 좌표 -> 0~100 비례 투영. span === 0이면 0으로 나누지 않고 50(중앙)에 둔다. */
function projectAxis(value: number, min: number, max: number): number {
  const span = max - min;
  if (span === 0) return 50;
  return ((value - min) / span) * 100;
}

function formatPercent(value: number): string {
  return String(Math.round(value * 100) / 100);
}

// -- 핵심 로그 -----------------------------------------------------------------

/** 표시 상한 (정본 07: 전체 로그는 일시정지/전투 종료 후 별도 열람 — 이
 * 슬라이스는 전체 로그 열람 UI를 만들지 않는다). 넘으면 생략 개수를
 * 명시한다 (I7, 조용한 truncation 금지). */
const WEB_CORE_LOG_LIMIT = 40;

/** `core_log`만 문장화한다. `full_log`는 개수만 표시한다. */
export function renderCombatLog(view: CombatSpectatorView): string {
  const metaText = `전체 로그 ${view.full_log.length}건 (일시정지 또는 전투 종료 후 별도 열람, 이 화면은 개수만 표시)`;
  const total = view.core_log.length;
  const shown = Math.min(total, WEB_CORE_LOG_LIMIT);
  const rows = view.core_log.slice(0, shown).map(renderLogRow).join('');
  const emptyLine = total === 0 ? '<li class="combat-log__empty">핵심 로그가 없다.</li>' : '';
  const omittedLine =
    total > shown ? `<li class="combat-log__omitted">…(생략 ${total - shown}줄)</li>` : '';

  return `<div class="combat-stage__log" data-region="combat-log" aria-label="전투 핵심 로그">
    <p class="combat-log__meta">${escapeHtml(metaText)}</p>
    <ol class="combat-log__list">${emptyLine}${rows}${omittedLine}</ol>
  </div>`;
}

/** `combat.log.damage_applied` 로그 줄만 Hit cue와 같은 색·같은 글리프를
 * 쓴다 (I11). core의 `cues_for`가 `Hit` cue를
 * `outcome.hit && outcome.damage_hundredths > 0`에서 만들고,
 * `damage_applied`는 같은 `DamageApplied` 사건에서 나오기 때문에 대응이
 * 증명 가능하다. 나머지 5개 template id는 대응하는 cue가 core에 없으므로
 * `data-cue`를 붙이지 않고 중립 잉크색으로 둔다 — 대응을 발명하지 않는다. */
function renderLogRow(entry: CombatSpectatorLogEntry): string {
  const line = combatLogTemplateLine(entry);
  const isDamageApplied = entry.template_id === 'combat.log.damage_applied';
  const cueAttr = isDamageApplied ? ' data-cue="hit"' : '';
  const cueGlyph = isDamageApplied
    ? `<span class="combat-log__cue-glyph" data-cue="hit" aria-hidden="true">${CUE_GLYPHS.hit}</span>`
    : '';
  const unknownAttr = isKnownCombatLogTemplateId(entry.template_id)
    ? ''
    : ' data-log-unknown="true"';
  return `<li class="combat-log__row" data-template-id="${escapeHtml(
    entry.template_id,
  )}"${cueAttr}${unknownAttr}>${cueGlyph}${escapeHtml(line)}</li>`;
}

// -- 전투 종료 보고서 ------------------------------------------------------------

const OUTCOME_LABELS: Record<CombatConclusionOutcome, string> = {
  in_progress: '진행 중',
  ally_victory: '아군 승리',
  enemy_victory: '적 승리',
  mutual_defeat: '양측 전멸',
  stalemate: '무승부',
};

const REASON_LABELS: Record<CombatConclusionReason, string> = {
  no_terminal_condition: '종료 조건 없음',
  all_enemies_defeated: '적 전멸',
  all_allies_defeated: '아군 전멸',
  both_sides_defeated: '양측 전멸',
  max_ticks_reached: '최대 tick 도달',
};

/** `combat.report`가 `Some`일 때만 호출된다 (호출부: `renderCombatStage`).
 * 금지: 전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투
 * 비교 — 이 함수는 `CombatConclusionReport` 필드를 그대로 옮기기만 한다. */
export function renderCombatReport(view: CombatSpectatorView, report: CombatConclusionReport): string {
  const survivors = report.survivor_ids.length
    ? report.survivor_ids.map(escapeHtml).join(', ')
    : '없음';
  const defeated = report.defeated_ids.length
    ? report.defeated_ids.map(escapeHtml).join(', ')
    : '없음';
  // 발생하지 않은 항목은 숨긴다: null이면 줄 자체를 만들지 않는다 (I8).
  const decisiveTickLine =
    report.decisive_tick !== null && report.decisive_tick !== undefined
      ? `<p class="combat-report__row">결착 tick: ${String(report.decisive_tick)}</p>`
      : '';
  const topDealtLine = report.top_damage_dealt_id
    ? `<p class="combat-report__row">최대 피해를 가한 전투원: ${escapeHtml(report.top_damage_dealt_id)}</p>`
    : '';
  const topTakenLine = report.top_damage_taken_id
    ? `<p class="combat-report__row">최대 피해를 받은 전투원: ${escapeHtml(report.top_damage_taken_id)}</p>`
    : '';
  const combatantRows = report.combatants.length
    ? `<ul class="combat-report__combatants">${report.combatants.map(renderCombatantRow).join('')}</ul>`
    : '<p class="combat-report__empty">전투원 상세 기록 없음.</p>';

  // fingerprint를 표시하는 요소에는 simulation_version을 같은 요소 안에 둔다
  // (정본 03 비교 계약).
  return `<section class="combat-report" data-region="combat-report" aria-label="전투 종료 보고서">
    <p class="combat-report__fingerprint">시뮬레이션 버전: ${escapeHtml(view.simulation_version)} · 지문: ${escapeHtml(
    report.fingerprint,
  )}</p>
    <p class="combat-report__row">결과: ${escapeHtml(OUTCOME_LABELS[report.outcome])}</p>
    <p class="combat-report__row">사유: ${escapeHtml(REASON_LABELS[report.reason])}</p>
    ${decisiveTickLine}
    <p class="combat-report__row">전투 시간: ${String(report.duration_millis)}ms</p>
    <p class="combat-report__row">생존: ${survivors}</p>
    <p class="combat-report__row">전투불능: ${defeated}</p>
    ${topDealtLine}
    ${topTakenLine}
    ${combatantRows}
  </section>`;
}

function renderCombatantRow(combatant: CombatCombatantReport): string {
  const dealt = roundHundredthsToInt(combatant.damage_dealt_hundredths);
  const taken = roundHundredthsToInt(combatant.damage_taken_hundredths);
  return `<li class="combat-report__combatant" data-combatant-id="${escapeHtml(
    combatant.id,
  )}">${escapeHtml(combatant.id)}: 가한 피해 ${dealt} · 받은 피해 ${taken} · 처치 ${
    combatant.kills
  } · 전투불능 ${combatant.incapacitated ? '예' : '아니오'}</li>`;
}

// -- 표면 통합 -------------------------------------------------------------------

/** `page.combat`이 `undefined`면 빈 문자열을 반환한다 — 래퍼 요소·클래스·
 * `data-*` 속성도 추가하지 않는다 (I5: 기존 51개 인카운터의 출력이 바이트
 * 단위로 그대로 유지되어야 한다). */
export function renderCombatStage(combat: CombatSpectatorPage | undefined): string {
  if (!combat) return '';
  const board = renderCombatBoard(combat.view);
  const log = renderCombatLog(combat.view);
  const report = combat.report ? renderCombatReport(combat.view, combat.report) : '';

  return `<section class="combat-stage" data-region="combat" aria-label="전투 관전">
    ${board}
    ${log}
  </section>
  ${report}`;
}
