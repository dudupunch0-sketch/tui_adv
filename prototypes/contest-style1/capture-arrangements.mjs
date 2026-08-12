#!/usr/bin/env node
/**
 * Capture the expression sheet to PNG.
 *
 * Same contract as the other harnesses: served over loopback, seeded, no
 * animation clock, so the same command always writes the same bytes.
 *
 * ```bash
 * PLAYWRIGHT_CHROMIUM_PATH=/path/to/chrome node capture-faces.mjs
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
  const file = path.join(here, rel === '/' ? 'arrangements.html' : rel);
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
  const page = await browser.newPage({ viewport: { width: 1660, height: 1400 }, deviceScaleFactor: 3 });
  const errors = [];
  page.on('pageerror', (err) => errors.push(String(err)));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  await page.goto(`http://127.0.0.1:${port}/arrangements.html`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
  await page.evaluate(() => document.fonts.ready);

  if (errors.length) {
    console.error('page errors:\n  ' + errors.join('\n  '));
    process.exitCode = 1;
  }

  const sheet = path.join(outDir, 'style1-arrangements.png');
  await page.locator('#sheet').screenshot({ path: sheet });
  shots.push(sheet);

  // Two cells pulled out on their own so occlusion order can be judged: the
  // contact sheet is at real phone size, which is the point, but that is also
  // too small to see which figure won a depth test.
  for (const key of ['stacked', 'crowded', 'surrounded']) {
    const file = path.join(outDir, `arrangement-${key}.png`);
    await page.locator(`#probe-${key}`).screenshot({ path: file });
    shots.push(file);
  }
} finally {
  await browser.close();
  server.close();
}

console.log(`captured ${shots.length} shots:`);
for (const s of shots) console.log('  ' + path.relative(process.cwd(), s));
