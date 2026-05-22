import type { SceneEffectCue, SceneVisual } from '../../core/types';
import { glyphFxFallbackText, glyphFxStableTerms } from '../../effects/glyphfx';
import { escapeHtml } from './html';

type VisualKind = 'messenger' | 'printer' | 'corridor' | 'placeholder';

const visualKinds = new Map<string, VisualKind>([
  ['opening_messenger', 'messenger'],
  ['encounter:ex_employee_messenger', 'messenger'],
  ['printer_anomaly', 'printer'],
  ['office_corridor_static', 'corridor'],
  ['location:hallway', 'corridor'],
]);

export function renderVisualCard(visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  const visualKind = visualKinds.get(visual.id) ?? 'placeholder';
  const body = renderVisualBody(visualKind, visual, effectCues);

  return `<figure class="storybook-visual storybook-visual--${visualKind}" data-region="visual" data-visual-id="${escapeHtml(
    visual.id,
  )}" data-visual-kind="${visualKind}">${body}</figure>`;
}

function renderVisualBody(visualKind: VisualKind, visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  if (visualKind === 'printer') return renderPrinterVisual(visual, effectCues);
  if (visualKind === 'messenger') return renderMessengerVisual(visual);
  if (visualKind === 'corridor') return renderCorridorVisual(visual);
  return renderPlaceholderVisual(visual);
}

function renderPrinterVisual(visual: SceneVisual, effectCues: SceneEffectCue[]): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="printer-card" aria-label="${escapeHtml(visual.alt)}">
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
    <div class="message-card" aria-label="${escapeHtml(visual.alt)}">
      <p>사내 메신저</p>
      <blockquote>아직 하지 않은 선택이 있습니다.</blockquote>
    </div>
  `;
}

function renderCorridorVisual(visual: SceneVisual): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="corridor-card" aria-label="${escapeHtml(visual.alt)}">
      <span>비상등</span><span>╱╲╱╲╱╲</span><span>EXIT?</span>
    </div>
  `;
}

function renderPlaceholderVisual(visual: SceneVisual): string {
  return `
    <figcaption>visual: ${escapeHtml(visual.id)}</figcaption>
    <div class="placeholder-card">
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
