#!/usr/bin/env node
/** Load a page and print its errors. `node debug.mjs playscreen.html` */
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright-chromium';

const here = path.dirname(fileURLToPath(import.meta.url));
const entry = process.argv[2] ?? 'playscreen.html';
const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.json': 'application/json' };
const server = createServer(async (req, res) => {
  const rel = decodeURIComponent((req.url ?? '/').split('?')[0]);
  const file = path.join(here, rel === '/' ? entry : rel);
  if (!file.startsWith(here)) return void res.writeHead(403).end('forbidden');
  try {
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': MIME[path.extname(file)] ?? 'application/octet-stream' }).end(body);
  } catch {
    res.writeHead(404).end('not found');
  }
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const browser = await chromium.launch(
  process.env.PLAYWRIGHT_CHROMIUM_PATH ? { executablePath: process.env.PLAYWRIGHT_CHROMIUM_PATH } : {},
);
const page = await browser.newPage();
page.on('pageerror', (e) => console.log('PAGEERROR: ' + String(e).split('\n').slice(0, 5).join('\n  ')));
page.on('console', (m) => {
  if (m.type() === 'error') console.log('CONSOLE: ' + m.text());
});
await page.goto(`http://127.0.0.1:${server.address().port}/${entry}`, { waitUntil: 'load' });
await page.waitForTimeout(4000);
console.log('READY =', await page.evaluate(() => window.__READY__));
await browser.close();
server.close();
