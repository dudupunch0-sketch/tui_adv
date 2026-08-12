#!/usr/bin/env node
/**
 * Capture the placement probe sheet.
 *
 * Same loopback contract as the other harnesses. Also prints the measured
 * figure height per arrangement, because "does the board scale collapse the
 * figures" deserves a number rather than an impression.
 */

import { createServer } from 'node:http';
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright-chromium';

const here = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.resolve(
  (process.argv.find((a) => a.startsWith('--out=')) ?? '--out=shots-arr').slice('--out='.length),
);

const MIME = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
};

const server = createServer(async (req, res) => {
  const rel = decodeURIComponent((req.url ?? '/').split('?')[0]);
  if (rel === '/favicon.ico') return void res.writeHead(204).end();
  const file = path.join(here, rel === '/' ? 'arrangements.html' : rel);
  if (!file.startsWith(here)) return void res.writeHead(403).end('forbidden');
  try {
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': MIME[path.extname(file)] ?? 'application/octet-stream' }).end(body);
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
  const page = await browser.newPage({ viewport: { width: 1440, height: 1200 }, deviceScaleFactor: 2 });
  const errors = [];
  page.on('pageerror', (err) => errors.push(String(err)));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  await page.goto(`http://127.0.0.1:${port}/arrangements.html`, { waitUntil: 'load' });
  await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 45000 });
  if (errors.length) {
    console.error('page errors:\n  ' + errors.join('\n  '));
    process.exitCode = 1;
  }

  console.table(await page.evaluate(() => window.__PROBE__));

  const sheet = path.join(outDir, 'style2-arrangements.png');
  await page.locator('#sheet').screenshot({ path: sheet });
  shots.push(sheet);

  // A 2× pass over the two arrangements most likely to hide a defect: the
  // depth-aligned column and the six-way melee.
  const zoom = await browser.newPage({ viewport: { width: 1440, height: 1200 }, deviceScaleFactor: 4 });
  await zoom.goto(`http://127.0.0.1:${port}/arrangements.html`, { waitUntil: 'load' });
  await zoom.waitForFunction(() => window.__READY__ === true, null, { timeout: 45000 });
  for (const [n, name] of [
    [4, 'zoom-stacked-column'],
    [7, 'zoom-crowded'],
  ]) {
    const file = path.join(outDir, `${name}.png`);
    await zoom.locator(`.grid figure:nth-child(${n}) .stage`).screenshot({ path: file });
    shots.push(file);
  }
} finally {
  await browser.close();
  server.close();
}

console.log(`captured ${shots.length} shots:`);
for (const s of shots) console.log('  ' + path.relative(process.cwd(), s));
