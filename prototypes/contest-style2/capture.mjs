#!/usr/bin/env node
/**
 * Capture the character-rig style prototypes to PNG.
 *
 * The point of committing this rather than running it by hand: a visual claim
 * ("style B reads better at mobile size") is only checkable if anyone can
 * reproduce the exact frame it was made from. The board is rendered from real
 * `ScenePage.combat` output (see `combat-frames.json`, produced by
 * `cargo run -p escape-core --example dump_combat_spectator`), the RNG is
 * seeded, and there is no animation clock — so the same command always writes
 * the same bytes.
 *
 * ```bash
 * node prototypes/character-rig/capture.mjs
 * node prototypes/character-rig/capture.mjs --out=/tmp/shots
 * ```
 */

import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { mkdir } from 'node:fs/promises';
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

// ES modules and fetch() both refuse to work from file://, so the page is
// served over loopback for the duration of the capture.
const server = createServer(async (req, res) => {
  const rel = decodeURIComponent((req.url ?? '/').split('?')[0]);
  // The browser asks for a favicon on every load; answering 204 keeps that out
  // of the harness's error list, where it would look like a real failure.
  if (rel === '/favicon.ico') {
    res.writeHead(204).end();
    return;
  }
  const file = path.join(here, rel === '/' ? 'index.html' : rel);
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

// `PLAYWRIGHT_CHROMIUM_PATH` lets a machine with a pre-installed browser skip
// the per-version download Playwright would otherwise insist on. Without it
// the default resolution applies.
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_PATH || undefined;
const browser = await chromium.launch(executablePath ? { executablePath } : {});
const shots = [];
try {
  const page = await browser.newPage({ viewport: { width: 1120, height: 900 }, deviceScaleFactor: 2 });
  const errors = [];
  page.on('pageerror', (err) => errors.push(String(err)));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });

  if (errors.length) {
    console.error('page errors:\n  ' + errors.join('\n  '));
    process.exitCode = 1;
  }

  const sheet = path.join(outDir, 'comparison-sheet.png');
  await page.locator('#sheet').screenshot({ path: sheet });
  shots.push(sheet);

  for (const id of ['1-bone2d', '2-cel3d']) {
    const file = path.join(outDir, `${id}.png`);
    await page.locator(`#detail-${id}`).screenshot({ path: file });
    shots.push(file);
  }
} finally {
  await browser.close();
  server.close();
}

console.log(`captured ${shots.length} shots:`);
for (const s of shots) console.log('  ' + path.relative(process.cwd(), s));
