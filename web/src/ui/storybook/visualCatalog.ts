import type { SceneEffectCue, SceneVisual } from '../../core/types';
import { glyphFxFallbackText, glyphFxStableTerms } from '../../effects/glyphfx';
import { escapeHtml } from './html';

type VisualKind = 'messenger' | 'printer' | 'corridor' | 'wuxia' | 'combat' | 'placeholder';

const visualKinds = new Map<string, VisualKind>([
  ['opening_messenger', 'messenger'],
  ['encounter:ex_employee_messenger', 'messenger'],
  ['printer_anomaly', 'printer'],
  ['office_corridor_static', 'corridor'],
  ['location:hallway', 'corridor'],
  ['wuxia_commute_rift', 'wuxia'],
  ['wuxia_cheonggi_record_first_fragment', 'wuxia'],
  ['wuxia_seo_harin_rescue', 'wuxia'],
  ['wuxia_cheongryu_apprentice_entry', 'wuxia'],
  ['wuxia_heuksa_bang_first_fight', 'combat'],
  ['supply_closet_scuffle', 'combat'],
]);

export function renderVisualCard(visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  const visualKind = visualKinds.get(visual.id) ?? visualKindFromLayout(visual);
  const body = renderVisualBody(visualKind, visual, effectCues);

  return `<figure class="storybook-visual storybook-visual--${visualKind}" data-region="visual" data-visual-id="${escapeHtml(
    visual.id,
  )}" data-visual-kind="${visualKind}">${body}</figure>`;
}

function renderVisualBody(visualKind: VisualKind, visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  if (visualKind === 'printer') return renderPrinterVisual(visual, effectCues);
  if (visualKind === 'messenger') return renderMessengerVisual(visual);
  if (visualKind === 'corridor') return renderCorridorVisual(visual);
  if (visualKind === 'wuxia') return renderWuxiaVisual(visual, effectCues);
  if (visualKind === 'combat') return renderCombatVisual(visual, effectCues);
  return renderPlaceholderVisual(visual);
}

function visualKindFromLayout(visual: SceneVisual): VisualKind {
  if (visual.kind === 'combat_intervention' || visual.id.includes('brawl') || visual.id.includes('fight')) {
    return 'combat';
  }
  if (
    visual.id.startsWith('wuxia_') ||
    visual.id.includes('jianghu') ||
    visual.id.includes('cheonggi') ||
    visual.id.includes('cheongryu')
  ) {
    return 'wuxia';
  }
  return 'placeholder';
}

function renderPrinterVisual(visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="pixel-illustration printer-card" aria-label="${escapeHtml(visual.alt)}">
      <canvas data-anomaly-canvas="printer-flow" aria-label="복합기 GlyphFX 장면"></canvas>
      <pre aria-hidden="true">      ________
 ____/ PRINT /__
| 복합기 | ▒▒▒ |  비상계단
|________|____|</pre>
    </div>
    ${renderGlyphFxCues(effectCues)}
  `;
}

function renderMessengerVisual(visual: SceneVisual): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="pixel-illustration message-card" aria-label="${escapeHtml(visual.alt)}">
      <p>사내 메신저</p>
      <blockquote>아직 하지 않은 선택이 있습니다.</blockquote>
    </div>
  `;
}

function renderCorridorVisual(visual: SceneVisual): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="pixel-illustration corridor-card" aria-label="${escapeHtml(visual.alt)}">
      <span>비상등</span><span>╱╲╱╲╱╲</span><span>EXIT?</span>
    </div>
  `;
}

function renderWuxiaVisual(visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="pixel-illustration wuxia-card" aria-label="${escapeHtml(visual.alt)}">
      <span class="wuxia-sky" aria-hidden="true"></span>
      <span class="wuxia-cliff" aria-hidden="true"></span>
      <span class="wuxia-gate" aria-hidden="true"></span>
      <span class="wuxia-traveler" aria-hidden="true"></span>
    </div>
    ${renderGlyphFxCues(effectCues)}
  `;
}

function renderCombatVisual(visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  const opponentName = visual.id.includes('wuxia') ? '흑사방 말단' : '상대';
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="pixel-illustration combat-card" aria-label="${escapeHtml(visual.alt)}">
      <div class="combat-versus" aria-hidden="true">
        <section class="combat-side combat-side--player">
          <strong>당신</strong>
          <span>거리 확보</span>
          <i></i>
        </section>
        <p><span>전투 발생</span><b>상황 개입</b></p>
        <section class="combat-side combat-side--enemy">
          <strong>${escapeHtml(opponentName)}</strong>
          <span>위협 높음</span>
          <i></i>
        </section>
      </div>
      <div class="combat-arena" aria-hidden="true">
        <span class="combat-hero"></span>
        <span class="combat-impact"></span>
        <span class="combat-enemy"></span>
      </div>
      <p class="combat-hint">선택지로 거리, 균형, 도주로를 고른다</p>
    </div>
    ${renderGlyphFxCues(effectCues)}
  `;
}

function renderPlaceholderVisual(visual: SceneVisual): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="pixel-illustration placeholder-card" aria-label="${escapeHtml(visual.alt || '아직 카탈로그에 없는 장면이다.')}">
      <p>${escapeHtml(visual.alt || '아직 카탈로그에 없는 장면이다.')}</p>
    </div>
  `;
}

function renderGlyphFxCues(effectCues: SceneEffectCue[]): string {
  const glyphCues = effectCues.filter((cue) => cue.kind === 'glyph_anomaly');
  if (!glyphCues.length) return '';

  return `<div class="storybook-glyphfx" aria-label="GlyphFX reduced motion fallback">${glyphCues
    .map(
      (cue) => `<section data-effect-kind="${escapeHtml(cue.kind)}" data-effect-source="${escapeHtml(cue.source)}">
        <p>${escapeHtml(glyphFxFallbackText(cue))}</p>
        <ul>${glyphFxStableTerms(cue)
          .map((term) => `<li><mark>${escapeHtml(term)}</mark></li>`)
          .join('')}</ul>
      </section>`,
    )
    .join('')}</div>`;
}
