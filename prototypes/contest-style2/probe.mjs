#!/usr/bin/env node
/** Run `calibrate.html` and print its result. Throwaway diagnostic, kept
 *  because the answer it gives is the single most load-bearing number in the
 *  renderer and re-deriving it by eye costs a capture round. */
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright-chromium';

const here = path.dirname(fileURLToPath(import.meta.url));
const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.json': 'application/json' };
const server = createServer(async (req, res) => {
  const rel = decodeURIComponent((req.url ?? '/').split('?')[0]);
  const file = path.join(here, rel === '/' ? 'calibrate.html' : rel);
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
page.on('pageerror', (e) => console.error('ERR', String(e)));
await page.goto(`http://127.0.0.1:${server.address().port}/`);
await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 20000 });
console.log(await page.textContent('#o'));
await browser.close();
server.close();
