import type { SceneEffectCue, SceneVisual } from '../../../core/types';
import { glyphFxFallbackText, glyphFxStableTerms } from '../../../effects/glyphfx';
import { escapeHtml } from '../html';
import { renderInkElement, renderInkFigure, renderMist } from './inkPrimitives';
import { sceneForVisual } from './inkScenes';
import { fnv1a, jitter, type InkSceneSpec } from './inkSpec';
import { artAssetFor } from '../art/artManifest';

export function renderInkVisual(visual: SceneVisual, effectCues: SceneEffectCue[], mode: string): string {
  const resolved = sceneForVisual(visual.id, mode);
  const known = resolved !== undefined;
  const spec = resolved ?? genericScene(mode);
  const artUrl = artAssetFor(visual.id);
  const hasArt = artUrl !== undefined;
  const kind = hasArt ? 'art' : isCombat(visual) ? 'combat' : known ? 'ink' : 'placeholder';
  const seed = fnv1a(visual.id);
  const layer = (elements: InkSceneSpec['far'], opacity: number) => (elements ?? []).map((element, index) => `<g opacity="${opacity}">${renderInkElement(element, jitter(seed, index, 0.04))}</g>`).join('');
  const figures = (spec.figures ?? []).map((figure, index) => renderInkFigure(figure, jitter(seed, index + 32, 0.04))).join('');
  const seal = spec.seal ? `<g class="ink-scene__seal" aria-hidden="true" transform="rotate(3 248 140)"><rect x="232" y="124" width="28" height="28" fill="${spec.accent === 'gold' ? '#b98f2e' : '#9e3c3f'}"/><text x="246" y="146" text-anchor="middle" fill="#f3ecd6" font-size="17">${escapeHtml(spec.seal)}</text></g>` : '';

  const svgContent = `<svg viewBox="0 0 280 168" preserveAspectRatio="xMidYMid slice" role="img" aria-label="${escapeHtml(visual.alt || '수묵 장면')}"><defs><filter id="ink-mist"><feGaussianBlur stdDeviation="4"/></filter></defs><rect width="280" height="168" fill="#e9dfc6"/>${layer(spec.far, 0.36)}${renderMist(spec.mist)}${layer(spec.mid, 0.6)}${layer(spec.near, 0.92)}${figures}${seal}</svg>`;
  const wrapperContent = hasArt
    ? `<div class="ink-scene__wrapper">
        <img class="ink-scene__art" src="${escapeHtml(artUrl)}" alt="" loading="lazy" decoding="async" width="1120" height="672" onload="this.classList.add('loaded'); this.closest('.ink-scene').setAttribute('data-art-status', 'loaded')" onerror="this.style.display='none'; this.closest('.ink-scene').setAttribute('data-art-status', 'failed')" />
        ${svgContent}
      </div>`
    : svgContent;

  const captionBar = (visual.alt || effectCues.some(c => c.kind === 'glyph_anomaly'))
    ? `<div class="ink-scene__caption-bar">
        ${visual.alt ? `<figcaption>${escapeHtml(visual.alt)}</figcaption>` : ''}
        ${renderGlyphFxCues(effectCues)}
      </div>`
    : '';

  return `<figure class="storybook-visual ink-scene ink-scene--${kind}${spec.night ? ' ink-scene--night' : ''}" data-region="visual" data-visual-id="${escapeHtml(visual.id)}" data-visual-kind="${kind}">
    ${captionBar}
    ${wrapperContent}
    ${visual.id === 'printer_anomaly' ? '<canvas data-anomaly-canvas="printer-flow" aria-label="복합기 GlyphFX 장면"></canvas>' : ''}
  </figure>`;
}

function genericScene(mode: string): InkSceneSpec {
  if (mode === 'ending') return { mist: 1, mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.55 }], seal: '記' };
  if (mode === 'movement') return { mist: 1, near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'walk', x: 0.5 }], seal: '行' };
  return { mist: 2, mid: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'confront', x: 0.45 }, { pose: 'stand', x: 0.65 }], seal: '問' };
}

function isCombat(visual: SceneVisual): boolean {
  return visual.kind === 'combat_intervention' || /brawl|fight|duel|battle/.test(visual.id);
}

function renderGlyphFxCues(effectCues: SceneEffectCue[]): string {
  const cues = effectCues.filter((cue) => cue.kind === 'glyph_anomaly');
  return cues.length ? `<div class="storybook-glyphfx" aria-label="GlyphFX reduced motion fallback">${cues.map((cue) => `<section data-effect-kind="${escapeHtml(cue.kind)}" data-effect-source="${escapeHtml(cue.source)}"><p>${escapeHtml(glyphFxFallbackText(cue))}</p><ul>${glyphFxStableTerms(cue).map((term) => `<li><mark>${escapeHtml(term)}</mark></li>`).join('')}</ul></section>`).join('')}</div>` : '';
}
