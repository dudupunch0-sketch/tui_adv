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
import { buildCombatMotionCss } from './combatMotion';
import type { PieceMotionTrack } from './combatMotion';

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-2 — 관전 표면 렌더러. Step 1d-3 — 재생 연출(모션) 배선.
//
// Hard invariant: this module only *formats* `page.combat`. It never
// recomputes damage, hit/miss, win/loss, cue derivation, log-importance
// filtering, damage totals, or "most damage" selection — escape-core has
// already decided all of that (정본 13). The only math performed here is
// display conversion: hundredths -> rounded int, tick -> elapsed ms,
// coordinate -> board percentage projection, array length display.
//
// Step 1d-3 adds one more piece of display conversion: per-tick projected
// coordinates -> a generated CSS `<style>` block (`combatMotion.ts` builds
// the actual `@keyframes` text; this module only decides *which* frames/
// coordinates feed it). See `combatMotion.ts`'s module comment for the full
// I1–I5/I9 invariant list that generator satisfies.
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

/** `view.frames`의 마지막 프레임을 그린다(정지 상태·`reduce` 최종 상태 —
 * 1d-2가 확보). 좌표 투영 범위는 Step 1d-3부터 **전체 프레임**의 말 min/max로
 * 확장한다: 마지막 프레임만으로 범위를 잡으면 재생 중 이동하는 말이 그 범위
 * 밖의 좌표를 지나칠 때 보드를 벗어나 보인다(§4-2). `span === 0`이면 0으로
 * 나누지 않고 50%(중앙)에 둔다.
 *
 * 이 투영 범위 확장은 로직 변경이 아니라 1d-2가 이미 쓰던 `projectAxis`의
 * 입력을 넓히는 것뿐이다 — 전투원이 매 프레임 같은 좌표에 머무는 저작
 * 시나리오(현재 1d-2 테스트 대부분)에서는 min/max가 그대로이므로 기존 기대값이
 * 바뀌지 않는다. */
export function renderCombatBoard(view: CombatSpectatorView): string {
  const frames = view.frames;
  const frame = frames.length ? frames[frames.length - 1] : undefined;

  if (!frame) {
    return `<div class="combat-stage__board" data-region="combat-board" role="img" aria-label="전투 판, 표시할 프레임이 없다">
      <p class="combat-board__empty">표시할 프레임이 없다.</p>
    </div>
    <table class="combat-board__table sr-only"><caption>전투 판 요약 표</caption><thead><tr><th scope="col">말 id</th><th scope="col">진영</th><th scope="col">좌표</th><th scope="col">참전</th><th scope="col">cue</th></tr></thead><tbody></tbody></table>`;
  }

  if (!frame.pieces.length) {
    return `<div class="combat-stage__board" data-region="combat-board" role="img" aria-label="전투 판, tick ${frame.tick}, 표시할 말이 없다">
      <p class="combat-board__empty">표시할 말이 없다 (전투원 0명).</p>
    </div>
    <table class="combat-board__table sr-only"><caption>전투 판 요약 표</caption><thead><tr><th scope="col">말 id</th><th scope="col">진영</th><th scope="col">좌표</th><th scope="col">참전</th><th scope="col">cue</th></tr></thead><tbody></tbody></table>`;
  }

  const allPieces = frames.flatMap((f) => f.pieces);
  const xs = allPieces.map((p) => p.position.x);
  const ys = allPieces.map((p) => p.position.y);
  const minX = Math.min(...xs);
  const maxX = Math.max(...xs);
  const minY = Math.min(...ys);
  const maxY = Math.max(...ys);

  const pieces = frame.pieces;
  const pieceMarkup = pieces
    .map((p) => renderPiece(p, projectAxis(p.position.x, minX, maxX), projectAxis(p.position.y, minY, maxY)))
    .join('');

  const allyCount = pieces.filter((p) => p.side === 'ally').length;
  const enemyCount = pieces.filter((p) => p.side === 'enemy').length;
  const boardLabel = `전투 판, tick ${frame.tick}, 아군 ${allyCount}명 · 적군 ${enemyCount}명`;

  const motionStyle = renderMotionStyleBlock(view, minX, maxX, minY, maxY);

  return `${motionStyle}<div class="combat-stage__board" data-region="combat-board" role="img" aria-label="${escapeHtml(boardLabel)}">
    ${pieceMarkup}
  </div>
  ${renderBoardTable(pieces)}`;
}

/** 재생 연출 CSS를 만든다(Step 1d-3). 실제 keyframe 문자열 생성은
 * `combatMotion.ts`가 전담한다 — 여기서는 어떤 프레임·좌표를 넘길지만
 * 결정한다(I4: 판정 재계산 0회).
 *
 * `<style>` 요소를 `<body>` 안에 두는 것은 HTML 스펙상 엄격히는 비적합이다
 * (`style`은 metadata content). 모든 브라우저가 그래도 적용하고 기능 문제는
 * 없다 — `renderStorybookPage`가 문자열 렌더러라 마운트 훅이 없으므로
 * `element.animate()`(WAAPI)를 쓰려면 `web/src/main.ts` 배선이 필요하고,
 * `prefers-reduced-motion`을 JS로 다시 확인해야 하며, 결정론적 단위 테스트로
 * 검증하기 어렵다(§4-1). 데이터에서 만든 `<style>`을 렌더 결과에 함께
 * 방출하면 문자열 렌더러 구조를 그대로 쓰면서 미디어 쿼리로 네이티브
 * 처리되고, 생성된 CSS 문자열이 결정론적이므로 단위 테스트로 총 길이·오프셋
 * 좌표를 그대로 고정할 수 있다. 이 트레이드오프를 여기 코드 주석과
 * `docs/design/Mobile_Ink_Storybook_UI.md`에 남긴다. */
function renderMotionStyleBlock(
  view: CombatSpectatorView,
  minX: number,
  maxX: number,
  minY: number,
  maxY: number,
): string {
  const tracks = collectPieceMotionTracks(view.frames, minX, maxX, minY, maxY);
  const motion = buildCombatMotionCss({ tickMillis: view.tick_millis, tracks });
  return motion.css ? `<style>${motion.css}</style>` : '';
}

/** 말 하나의 트랙은 그 말이 **모든** 프레임에 등장할 때만 만든다. 현재 core
 * 출력은 매 tick 같은 전투원 집합을 유지하며(퇴장 없이 `active`만 바뀐다)
 * 이 경우가 항상이지만, 만에 하나 어떤 tick의 `pieces`에 말이 아예 없으면 그
 * tick의 좌표를 지어낼 수 없으므로(I2: 임의의 waypoint 금지) 그 말은
 * 애니메이션 없이 마지막 프레임 정지 위치로만 남긴다. */
function collectPieceMotionTracks(
  frames: CombatSpectatorView['frames'],
  minX: number,
  maxX: number,
  minY: number,
  maxY: number,
): PieceMotionTrack[] {
  if (frames.length <= 1) return [];
  const lastFrame = frames[frames.length - 1];
  const tracks: PieceMotionTrack[] = [];
  for (const piece of lastFrame.pieces) {
    const points: PieceMotionTrack['frames'] = [];
    let presentInEveryFrame = true;
    for (const f of frames) {
      const match = f.pieces.find((p) => p.id === piece.id);
      if (!match) {
        presentInEveryFrame = false;
        break;
      }
      points.push({
        x: projectAxis(match.position.x, minX, maxX),
        y: projectAxis(match.position.y, minY, maxY),
        // WP3: carry this tick's own cue set/facing through verbatim — the
        // cue presentation grammar (combatMotion.ts) never infers either
        // from a neighboring tick (I2/I4).
        cues: match.cues,
        facing: match.facing,
      });
    }
    if (presentInEveryFrame) {
      tracks.push({ pieceId: piece.id, frames: points });
    }
  }
  return tracks;
}

function renderPiece(piece: CombatSpectatorPiece, xPercent: number, yPercent: number): string {
  const cueAttrs = piece.cues.map((cue) => ` data-cue-${escapeHtml(cue)}="true"`).join('');
  // 말 하나가 cue를 여러 개 가질 수 있다 (예: 피격 + 균형 붕괴 + 전투불능).
  // 표식을 각각 절대 배치하면 같은 자리에 겹쳐 마지막 하나만 보인다. 한
  // 컨테이너에 담아 나란히 놓는다.
  const cueSpans = piece.cues.length
    ? `<span class="combat-board__cues" aria-hidden="true">${piece.cues
        .map(
          (cue) =>
            `<span class="combat-board__cue" data-cue="${escapeHtml(cue)}">${CUE_GLYPHS[cue]}</span>`,
        )
        .join('')}</span>`
    : '';
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
      // `active`는 생존 여부가 아니다 — 정본 09의 "활성 전투"(완전 시뮬레이션)
      // 참가 여부이며 authoring 값에서 온다. 실측 확인: 체력이 0이 된 뒤에도
      // `active`는 계속 true이고, 전투불능은 `Incapacitated` cue로만 나타난다.
      // 그래서 이 칸을 "생존"이라고 쓰면 거짓이 된다. 생존/전투불능은
      // 보고서의 `survivor_ids`/`defeated_ids`가 소유한다.
      const participation = p.active ? '참전' : '비참전';
      const cueText = p.cues.length ? p.cues.map((cue) => CUE_LABELS[cue]).join(', ') : '없음';
      return `<tr><td>${escapeHtml(p.id)}</td><td>${escapeHtml(SIDE_LABELS[p.side])}</td><td>(${String(
        p.position.x,
      )}, ${String(p.position.y)})</td><td>${escapeHtml(participation)}</td><td>${escapeHtml(cueText)}</td></tr>`;
    })
    .join('');
  return `<table class="combat-board__table sr-only"><caption>전투 판 요약 표</caption><thead><tr><th scope="col">말 id</th><th scope="col">진영</th><th scope="col">좌표</th><th scope="col">참전</th><th scope="col">cue</th></tr></thead><tbody>${rows}</tbody></table>`;
}

/** 말은 `translate: -50% -50%`로 중심을 좌표에 맞추므로, 투영 범위가
 * 0~100%면 최소·최대 좌표의 말이 보드 경계에서 절반 잘린다. 전투원 2명인
 * 인카운터는 두 말이 항상 양 극단에 놓이므로 예외가 아니라 기본 경우다.
 * 그래서 여백을 둔 띠(14~86%) 안으로 투영한다. */
const BOARD_INSET_PERCENT = 14;

/** 좌표 -> 비례 투영. 대칭 여백이므로 span === 0이면 정확히 50(중앙)이다.
 * span === 0에서 0으로 나누지 않는다. 이 투영은 배치 비율일 뿐이며 거리·속도를
 * 수치로 주장하지 않는다 (좌표 단위 의미는 정본에 확정되지 않았다). */
function projectAxis(value: number, min: number, max: number): number {
  const span = max - min;
  if (span === 0) return 50;
  const usable = 100 - BOARD_INSET_PERCENT * 2;
  return BOARD_INSET_PERCENT + ((value - min) / span) * usable;
}

function formatPercent(value: number): string {
  return String(Math.round(value * 100) / 100);
}

// -- 핵심 로그 -----------------------------------------------------------------

/** 표시 상한 (정본 07: 전체 로그는 일시정지/전투 종료 후 별도 열람 — 이
 * 슬라이스는 전체 로그 열람 UI를 만들지 않는다). 넘으면 생략 개수를
 * 명시한다 (I7, 조용한 truncation 금지). */
const WEB_CORE_LOG_LIMIT = 40;

/** `core_log`만 문장화한다. `full_log`는 개수만 표시한다.
 *
 * Step 1d-3 (WP4): 각 줄의 노출 시각은 `entry.tick × view.tick_millis`다
 * (I6). `sequence` 순서는 core가 만든 `core_log` 배열 순서 그대로 유지한다
 * (`.slice`/`.map`은 재정렬하지 않는다) — 같은 tick의 여러 줄이 뒤섞이지
 * 않는다. 노출 전에도 DOM에서 제거하지 않는다: `animation-delay`로 opacity만
 * 늦추므로(storybook.css의 `.combat-log__row` 규칙), 스크린리더는 처음부터
 * 전체 로그를 읽을 수 있고 `full_log` 개수 표시와도 어긋나지 않는다.
 * `aria-live`는 쓰지 않는다 — 초당 여러 줄이 붙으면 로그 도배가 된다(정본
 * 13의 "로그 도배를 막는다"와 같은 취지). `reduce`에서는 `no-preference`
 * 안에만 있는 이 애니메이션 자체가 적용되지 않으므로 전부 즉시 보인다(I3). */
export function renderCombatLog(view: CombatSpectatorView): string {
  const metaText = `전체 로그 ${view.full_log.length}건 (일시정지 또는 전투 종료 후 별도 열람, 이 화면은 개수만 표시)`;
  const total = view.core_log.length;
  const shown = Math.min(total, WEB_CORE_LOG_LIMIT);
  const rows = view.core_log
    .slice(0, shown)
    .map((entry) => renderLogRow(entry, view.tick_millis))
    .join('');
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
function renderLogRow(entry: CombatSpectatorLogEntry, tickMillis: number): string {
  const line = combatLogTemplateLine(entry);
  const isDamageApplied = entry.template_id === 'combat.log.damage_applied';
  const cueAttr = isDamageApplied ? ' data-cue="hit"' : '';
  const cueGlyph = isDamageApplied
    ? `<span class="combat-log__cue-glyph" data-cue="hit" aria-hidden="true">${CUE_GLYPHS.hit}</span>`
    : '';
  const unknownAttr = isKnownCombatLogTemplateId(entry.template_id)
    ? ''
    : ' data-log-unknown="true"';
  // `animation-delay`만 인라인으로 얹는다 — `animation`(이름·길이·이징) 자체는
  // storybook.css의 정적 `.combat-log__row` 규칙이 준다. 인라인 longhand가
  // 외부 shorthand보다 그 속성 하나만 우선하는 표준 캐스케이드를 그대로
  // 쓴다(WP1 keyframe 이름과 달리 매 행마다 다른 CSS 텍스트를 만들 필요가
  // 없다).
  const revealDelay = ` style="animation-delay: ${String(entry.tick * tickMillis)}ms"`;
  return `<li class="combat-log__row"${revealDelay} data-template-id="${escapeHtml(
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
