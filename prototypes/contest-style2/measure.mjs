#!/usr/bin/env node
/** Print the measured pixel size of the play screen's board stage. */
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright-chromium';

const here = path.dirname(fileURLToPath(import.meta.url));
const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.json': 'application/json' };
const server = createServer(async (req, res) => {
  const rel = decodeURIComponent((req.url ?? '/').split('?')[0]);
  const file = path.join(here, rel === '/' ? 'playscreen.html' : rel);
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
const page = await browser.newPage({ viewport: { width: 960, height: 1000 } });
await page.goto(`http://127.0.0.1:${server.address().port}/playscreen.html?faces=0`, { waitUntil: 'load' });
await page.waitForFunction(() => window.__READY__ === true, null, { timeout: 30000 });
console.log(
  JSON.stringify(
    await page.evaluate(() => {
      const r = document.querySelector('#screen-t4 .stage').getBoundingClientRect();
      return { w: Math.round(r.width), h: Math.round(r.height) };
    }),
  ),
);
await browser.close();
server.close();
