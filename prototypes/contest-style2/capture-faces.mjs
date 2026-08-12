#!/usr/bin/env node
/**
 * Capture the expression sheet.
 *
 * Same loopback-server contract as the other harnesses. Two passes: the sheet
 * as it would be read, and a 4× pass over a single portrait, because a face
 * defect that is invisible at 208px is exactly the kind that survives a review
 * and then shows up in the pitch deck.
 */

import { createServer } from 'node:http';
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright-chromium';

const here = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.resolve(
  (process.argv.find((a) => a.startsWith('--out=')) ?? '--out=shots-face').slice('--out='.length),
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
  const file = path.join(here, rel === '/' ? 'facesheet.html' : rel);
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

const browser = await chromium.launch(
  process.env.PLAYWRIGHT_CHROMIUM_PATH ? { executablePath: process.env.PLAYWRIGHT_CHROMIUM_PATH } : {},
);
const shots = [];
try {
  const page = await browser.newPage({ viewport: { width: 1200, height: 1100 }, deviceScaleFactor: 2 });
  const errors = [];
  page.on('pageerror', (err) => errors.push(String(err)));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  await page.goto(`http://127.0.0.1:${port}/facesheet.html`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  if (errors.length) {
    console.error('page errors:\n  ' + errors.join('\n  '));
    process.exitCode = 1;
  }

  const sheet = path.join(outDir, 'style2-character-sheet.png');
  await page.locator('#sheet').screenshot({ path: sheet });
  shots.push(sheet);

  const zoom = await browser.newPage({ viewport: { width: 1200, height: 1100 }, deviceScaleFactor: 6 });
  await zoom.goto(`http://127.0.0.1:${port}/facesheet.html`, { waitUntil: 'load' });
  await zoom.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  for (const [sel, name] of [
    ['#bands .band:nth-of-type(2) .row figure:nth-child(2) .tile', 'zoom-ally-attack'],
    ['#bands .band:nth-of-type(3) .row figure:nth-child(4) .tile', 'zoom-enemy-t4'],
    ['#bands .band:nth-of-type(1) .sil figure:nth-child(1) .tile', 'zoom-ally-figure'],
  ]) {
    const file = path.join(outDir, `${name}.png`);
    await zoom.locator(sel).screenshot({ path: file });
    shots.push(file);
  }
} finally {
  await browser.close();
  server.close();
}

console.log(`captured ${shots.length} shots:`);
for (const s of shots) console.log('  ' + path.relative(process.cwd(), s));
