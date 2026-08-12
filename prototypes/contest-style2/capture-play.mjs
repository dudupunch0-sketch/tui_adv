#!/usr/bin/env node
/**
 * Capture the style-2 play-screen mockup to PNG.
 *
 * Same contract as `capture.mjs`: the page is served over loopback (ES modules
 * and `fetch()` both refuse `file://`), the RNG is seeded, and there is no
 * animation clock — so the same command always writes the same bytes. The only
 * difference is the page and which elements get shot.
 *
 * ```bash
 * PLAYWRIGHT_CHROMIUM_PATH=/path/to/chrome node capture-play.mjs
 * node capture-play.mjs --out=/tmp/shots
 * ```
 */

import { createServer } from 'node:http';
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright-chromium';

const here = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.resolve(
  (process.argv.find((a) => a.startsWith('--out=')) ?? '--out=shots').slice('--out='.length),
);

const MIME = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
};

const server = createServer(async (req, res) => {
  const rel = decodeURIComponent((req.url ?? '/').split('?')[0]);
  if (rel === '/favicon.ico') {
    res.writeHead(204).end();
    return;
  }
  const file = path.join(here, rel === '/' ? 'playscreen.html' : rel);
  if (!file.startsWith(here)) {
    res.writeHead(403).end('forbidden');
    return;
  }
  try {
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': MIME[path.extname(file)] ?? 'application/octet-stream' });
    res.end(body);
  } catch {
    res.writeHead(404).end('not found');
  }
});

await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
const port = server.address().port;
await mkdir(outDir, { recursive: true });

const executablePath = process.env.PLAYWRIGHT_CHROMIUM_PATH || undefined;
const browser = await chromium.launch(executablePath ? { executablePath } : {});
const shots = [];
try {
  const page = await browser.newPage({ viewport: { width: 960, height: 1000 }, deviceScaleFactor: 2 });
  const errors = [];
  page.on('pageerror', (err) => errors.push(String(err)));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  await page.goto(`http://127.0.0.1:${port}/playscreen.html`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });

  if (errors.length) {
    console.error('page errors:\n  ' + errors.join('\n  '));
    process.exitCode = 1;
  }

  for (const [id, name] of [
    ['screen-t4', 'style2-playscreen-t4-exchange'],
    ['screen-t8', 'style2-playscreen-t8-decision'],
  ]) {
    const file = path.join(outDir, `${name}.png`);
    await page.locator(`#${id}`).screenshot({ path: file });
    shots.push(file);
  }

  // The faceless variant, from the same page with one query flag flipped, so
  // the comparison differs in exactly the thing being compared.
  const bare = await browser.newPage({ viewport: { width: 960, height: 1000 }, deviceScaleFactor: 2 });
  await bare.goto(`http://127.0.0.1:${port}/playscreen.html?faces=0`, { waitUntil: 'load' });
  await bare.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  for (const [id, name] of [
    ['screen-t4', 'style2-faceless-t4-exchange'],
    ['screen-t8', 'style2-faceless-t8-decision'],
  ]) {
    const file = path.join(outDir, `${name}.png`);
    await bare.locator(`#${id}`).screenshot({ path: file });
    shots.push(file);
  }
  const bareZoom = await browser.newPage({ viewport: { width: 960, height: 1000 }, deviceScaleFactor: 4 });
  await bareZoom.goto(`http://127.0.0.1:${port}/playscreen.html?faces=0`, { waitUntil: 'load' });
  await bareZoom.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  const bz = path.join(outDir, 'zoom-faceless-t4-board.png');
  await bareZoom.locator('#screen-t4 .stage').screenshot({ path: bz });
  shots.push(bz);

  // A 4× pass over the board alone. Reviewing a 3D pose inside a 412px-wide
  // phone frame is how a backwards limb survives three capture rounds; the
  // zoom exists so the figures can actually be judged.
  const zoom = await browser.newPage({ viewport: { width: 960, height: 1000 }, deviceScaleFactor: 4 });
  await zoom.goto(`http://127.0.0.1:${port}/playscreen.html`, { waitUntil: 'load' });
  await zoom.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  for (const [id, name] of [
    ['screen-t4', 'zoom-t4-board'],
    ['screen-t8', 'zoom-t8-board'],
  ]) {
    const file = path.join(outDir, `${name}.png`);
    await zoom.locator(`#${id} .stage`).screenshot({ path: file });
    shots.push(file);
  }
} finally {
  await browser.close();
  server.close();
}

console.log(`captured ${shots.length} shots:`);
for (const s of shots) console.log('  ' + path.relative(process.cwd(), s));
