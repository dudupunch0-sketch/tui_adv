import type { InkElement, InkFigure } from './inkSpec';

const ink = '#20242b';
const wash = '#3a352b';

export function renderInkElement(element: InkElement, jitterX: number): string {
  const x = (element.x + jitterX) * 280;
  const scale = element.scale ?? 1;
  const transform = `translate(${x.toFixed(1)} 0) scale(${element.flip ? -scale : scale} ${scale})`;
  const shapes: Record<InkElement['kind'], string> = {
    ridge: '<path d="M-62 122 Q-26 74 8 112 Q40 62 76 120 L76 168 L-62 168Z" fill="currentColor"/>',
    roofline: '<path d="M-42 126 L0 90 L42 126 M-30 126 V151 H30 V126" fill="none" stroke="currentColor" stroke-width="5"/>',
    gate: '<path d="M-25 148 V82 H25 V148 M-38 82 H38 M-31 72 H31" fill="none" stroke="currentColor" stroke-width="6"/>',
    pine: '<path d="M0 150 V88 M0 91 L-26 119 M0 102 L29 128 M0 80 L-17 101" fill="none" stroke="currentColor" stroke-width="5" stroke-linecap="round"/>',
    bamboo: '<path d="M-12 154 L-2 82 M8 154 L18 88 M-4 118 H15 M-7 102 H19" fill="none" stroke="currentColor" stroke-width="4" stroke-linecap="round"/>',
    road: '<path d="M-34 168 Q0 123 34 168" fill="none" stroke="currentColor" stroke-width="9" opacity=".7"/>',
    wall: '<path d="M-55 144 H55 M-45 144 V111 H45 V144 M-55 111 H55" fill="none" stroke="currentColor" stroke-width="5"/>',
    stairs: '<path d="M-36 154 H-16 V138 H4 V122 H24 V106 H44" fill="none" stroke="currentColor" stroke-width="5"/>',
    desk: '<path d="M-34 137 H34 M-25 137 V155 M25 137 V155" fill="none" stroke="currentColor" stroke-width="5"/>',
    lantern: '<path d="M0 112 V146 M-9 121 H9 M-7 121 V136 H7 V121" fill="none" stroke="currentColor" stroke-width="4"/>',
    campfire: '<path d="M-13 151 Q0 116 13 151 Q0 143-13 151Z" fill="currentColor"/>',
    banner: '<path d="M0 78 V151 M0 84 Q21 88 13 112 Q4 104 0 113" fill="none" stroke="currentColor" stroke-width="5"/>',
    scroll: '<path d="M-18 122 H18 V142 H-18Z M-18 122 Q-26 126-18 130 M18 134 Q26 138 18 142" fill="none" stroke="currentColor" stroke-width="3"/>',
    rain: '<path d="M-28 93 L-43 142 M0 82 L-15 146 M28 91 L13 145" fill="none" stroke="currentColor" stroke-width="2"/>',
    moon: '<circle cx="0" cy="80" r="13" fill="none" stroke="currentColor" stroke-width="3"/>',
  };
  return `<g transform="${transform}">${shapes[element.kind]}</g>`;
}

export function renderInkFigure(figure: InkFigure, jitterX: number): string {
  const x = (figure.x + jitterX) * 280;
  const scale = figure.scale ?? 1;
  const weapon = figure.weapon && figure.weapon !== 'none'
    ? `<path d="M12 111 L${figure.weapon === 'club' ? '28 96' : '34 78'}" stroke="${ink}" stroke-width="3" stroke-linecap="round"/>`
    : '';
  const pose: Record<InkFigure['pose'], string> = {
    stand: 'M0 100 Q-4 121 0 140 M0 116 L-12 129 M0 116 L12 128 M0 140 L-9 157 M0 140 L9 157',
    walk: 'M0 100 Q5 121 0 140 M0 116 L-14 126 M0 116 L13 122 M0 140 L-14 154 M0 140 L12 160',
    confront: 'M0 100 Q-2 121 0 140 M0 116 L-16 118 M0 116 L15 111 M0 140 L-10 157 M0 140 L10 157',
    kneel: 'M0 111 Q-6 128 4 140 M0 122 L-13 132 M4 140 L-12 148 M4 140 L16 148',
    fallen: 'M-18 143 Q0 132 22 145 M-2 132 L10 118',
    bow: 'M0 103 Q-14 119 -4 139 M-4 119 L-18 126 M-4 139 L-14 154 M-4 139 L8 154',
    'duel-left': 'M0 100 Q-7 121 0 140 M0 116 L-17 107 M0 116 L13 127 M0 140 L-11 157 M0 140 L9 157',
    'duel-right': 'M0 100 Q7 121 0 140 M0 116 L17 107 M0 116 L-13 127 M0 140 L11 157 M0 140 L-9 157',
    sit: 'M0 108 Q-6 126 0 139 M0 121 L-15 130 M0 139 L-14 148 M0 139 L14 148',
    reach: 'M0 100 Q-3 122 0 140 M0 115 L16 96 M0 115 L-12 126 M0 140 L-9 157 M0 140 L9 157',
  };
  return `<g transform="translate(${x.toFixed(1)} 0) scale(${scale})" fill="none" stroke="${ink}" stroke-width="4" stroke-linecap="round"><circle cx="0" cy="91" r="7" fill="${ink}" stroke="none"/><path d="${pose[figure.pose]}"/>${weapon}</g>`;
}

export function renderMist(level: number): string {
  return Array.from({ length: level }, (_, index) => `<ellipse cx="${72 + index * 67}" cy="${104 + index * 8}" rx="${54 - index * 6}" ry="12" fill="${wash}" opacity=".16" filter="url(#ink-mist)"/>`).join('');
}
