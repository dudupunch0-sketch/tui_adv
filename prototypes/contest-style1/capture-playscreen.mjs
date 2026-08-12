#!/usr/bin/env node
/**
 * Capture the style-1 combat play screen to PNG.
 *
 * Same contract as `capture.mjs`: the page is served over loopback (ES modules
 * and `fetch()` both refuse to work from `file://`), the RNG is seeded and
 * there is no animation clock, so the same command always writes the same
 * bytes.
 *
 * ```bash
 * PLAYWRIGHT_CHROMIUM_PATH=/path/to/chrome node capture-playscreen.mjs
 * node capture-playscreen.mjs --out=/tmp/shots
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
  const page = await browser.newPage({ viewport: { width: 1760, height: 990 }, deviceScaleFactor: 3 });
  const errors = [];
  page.on('pageerror', (err) => errors.push(String(err)));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  await page.goto(`http://127.0.0.1:${port}/playscreen.html`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  await page.evaluate(() => document.fonts.ready);

  if (errors.length) {
    console.error('page errors:\n  ' + errors.join('\n  '));
    process.exitCode = 1;
  }

  for (const [id, name] of [
    ['#phone-t4', 'style1-playscreen-t4.png'],
    ['#phone-t8', 'style1-playscreen-t8.png'],
    ['#detail-t4', 'detail-t4.png'],
    ['#detail-t8', 'detail-t8.png'],
  ]) {
    const file = path.join(outDir, name);
    await page.locator(id).screenshot({ path: file });
    shots.push(file);
  }

  const pair = path.join(outDir, 'style1-playscreen-pair.png');
  await page.screenshot({ path: pair, fullPage: true });
  shots.push(pair);

  // Faceless variant: the same page with `?faces=0`. Same poses, cloth, hair,
  // weapons, board, log and chrome — only the facial features are subtracted,
  // so the two sets of shots are directly comparable.
  await page.goto(`http://127.0.0.1:${port}/playscreen.html?faces=0`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  await page.evaluate(() => document.fonts.ready);
  for (const [id, name] of [
    ['#phone-t4', 'style1-faceless-t4.png'],
    ['#phone-t8', 'style1-faceless-t8.png'],
    ['#detail-t4', 'faceless-detail-t4.png'],
  ]) {
    const file = path.join(outDir, name);
    await page.locator(id).screenshot({ path: file });
    shots.push(file);
  }
} finally {
  await browser.close();
  server.close();
}

console.log(`captured ${shots.length} shots:`);
for (const s of shots) console.log('  ' + path.relative(process.cwd(), s));
