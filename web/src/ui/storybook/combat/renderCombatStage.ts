import type {
  CombatSpectatorCue,
  CombatSpectatorPiece,
  CombatSpectatorView,
  CombatSide,
} from '../../../core/types';
import { escapeHtml } from '../html';

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
