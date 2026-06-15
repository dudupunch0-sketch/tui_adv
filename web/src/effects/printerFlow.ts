import encountersData from '../data/generated/encounters.json';
import secretsData from '../data/generated/secrets.example.json';

export const PRETEXT_MODULE_URL = 'https://esm.sh/@chenglou/pretext@0.0.6';

export interface PrinterFlowScene {
  title: string;
  corpus: string;
  obstacle: {
    kind: 'copier';
    label: string;
  };
}

interface EncounterRecord {
  id: string;
  title?: string;
  body?: string;
}

interface SecretRecord {
  id: string;
  public_hint_steps?: string[];
  reward_text?: string;
}

export function buildPrinterFlowScene(): PrinterFlowScene {
  const encounter = (encountersData as EncounterRecord[]).find((candidate) => candidate.id === 'printer_prints_alone');
  const secret = (secretsData as SecretRecord[]).find((s) => s.id === 'real_note_001');
  if (!encounter || !secret) {
    throw new Error('printer flow scene requires printer encounter and real_note_001');
  }
  return {
    title: encounter.title ?? '',
    obstacle: { kind: 'copier', label: 'OFFICE COPIER / OUTPUT TRAY' },
    corpus: [
      encounter.title ?? '',
      encounter.body ?? '',
      ...(secret.public_hint_steps ?? []),
      secret.reward_text ?? '',
    ].join('\n'),
  };
}

export function renderPrinterFallbackText(scene: PrinterFlowScene): string {
  return [`[ANOMALY CANVAS FALLBACK] ${scene.title}`, scene.corpus].join('\n');
}

export async function loadPretextModule(): Promise<unknown> {
  return import(/* @vite-ignore */ PRETEXT_MODULE_URL);
}

export async function startPrinterFlowEffect(canvas: HTMLCanvasElement): Promise<void> {
  const scene = buildPrinterFlowScene();
  const context = canvas.getContext('2d');
  if (!context) return;
  sizeCanvas(canvas);
  drawStaticFrame(context, canvas, scene, false);
  try {
    const pretext = (await loadPretextModule()) as PretextLike;
    drawPretextFrame(context, canvas, scene, pretext);
  } catch {
    drawStaticFrame(context, canvas, scene, true);
  }
}

type PretextLike = {
  prepareWithSegments?: (text: string, font: string) => unknown;
  layoutWithLines?: (prepared: unknown, width: number, lineHeight: number) => { lines?: Array<{ text: string }> };
};

function sizeCanvas(canvas: HTMLCanvasElement): void {
  const rect = canvas.getBoundingClientRect();
  const width = Math.max(640, Math.floor(rect.width || 720));
  const height = Math.max(260, Math.floor(rect.height || 320));
  const ratio = window.devicePixelRatio || 1;
  canvas.width = width * ratio;
  canvas.height = height * ratio;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  const context = canvas.getContext('2d');
  context?.setTransform(ratio, 0, 0, ratio, 0, 0);
}

function drawPretextFrame(context: CanvasRenderingContext2D, canvas: HTMLCanvasElement, scene: PrinterFlowScene, pretext: PretextLike): void {
  const font = '15px "IBM Plex Mono", "JetBrains Mono", monospace';
  if (!pretext.prepareWithSegments || !pretext.layoutWithLines) {
    drawStaticFrame(context, canvas, scene, true);
    return;
  }
  const width = Number.parseInt(canvas.style.width, 10);
  const prepared = pretext.prepareWithSegments(scene.corpus, font);
  const layout = pretext.layoutWithLines(prepared, Math.max(280, width - 320), 22);
  drawBackdrop(context, canvas);
  drawCopierObstacle(context);
  context.font = font;
  context.fillStyle = '#d5ffe1';
  const lines = layout.lines ?? [];
  lines.slice(0, 11).forEach((line, index) => {
    const y = 56 + index * 22;
    const crossesCopier = y > 88 && y < 210;
    const x = crossesCopier ? 300 : 46;
    context.fillText(line.text, x, y);
  });
  drawScanline(context, canvas);
}

function drawStaticFrame(context: CanvasRenderingContext2D, canvas: HTMLCanvasElement, scene: PrinterFlowScene, fallback: boolean): void {
  drawBackdrop(context, canvas);
  drawCopierObstacle(context);
  context.font = '15px "IBM Plex Mono", "JetBrains Mono", monospace';
  context.fillStyle = fallback ? '#ffd479' : '#d5ffe1';
  const lines = renderPrinterFallbackText(scene).split('\n').slice(0, 11);
  lines.forEach((line, index) => {
    context.fillText(line, 300, 54 + index * 22);
  });
  drawScanline(context, canvas);
}

function drawBackdrop(context: CanvasRenderingContext2D, canvas: HTMLCanvasElement): void {
  const width = Number.parseInt(canvas.style.width, 10);
  const height = Number.parseInt(canvas.style.height, 10);
  context.clearRect(0, 0, width, height);
  context.fillStyle = '#030805';
  context.fillRect(0, 0, width, height);
  context.strokeStyle = 'rgba(121, 255, 163, 0.5)';
  context.strokeRect(10, 10, width - 20, height - 20);
  context.fillStyle = 'rgba(121, 255, 163, 0.16)';
  context.fillText('ANOMALY CANVAS READY / PRETEXT FLOW', 30, 30);
}

function drawCopierObstacle(context: CanvasRenderingContext2D): void {
  context.strokeStyle = '#79ffa3';
  context.fillStyle = 'rgba(121, 255, 163, 0.07)';
  context.lineWidth = 2;
  context.fillRect(38, 84, 220, 116);
  context.strokeRect(38, 84, 220, 116);
  context.strokeRect(68, 54, 160, 38);
  context.fillStyle = '#79ffa3';
  context.fillText('[COPIER]', 92, 78);
  context.fillText('OUTPUT >>>', 78, 146);
}

function drawScanline(context: CanvasRenderingContext2D, canvas: HTMLCanvasElement): void {
  const width = Number.parseInt(canvas.style.width, 10);
  const height = Number.parseInt(canvas.style.height, 10);
  context.fillStyle = 'rgba(255, 255, 255, 0.035)';
  for (let y = 0; y < height; y += 4) {
    context.fillRect(0, y, width, 1);
  }
}
