#!/usr/bin/env node
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium } from 'playwright-chromium';

const VIEWPORTS = [
  { name: '390x844', width: 390, height: 844 },
  { name: '414x896', width: 414, height: 896 },
  { name: '800x1440', width: 800, height: 1440 },
  { name: '810x1644', width: 810, height: 1644 },
  { name: '1440x1000', width: 1440, height: 1000 },
];

const REQUIRED_SELECTORS = [
  '[data-renderer="web-storybook"]',
  '.storybook-shell',
  '.storybook-hud[data-region="status"]',
  '.story-progress-rail',
  '[data-region="visual"]',
  '[data-region="body"]',
  '[data-region="choices"]',
  '[data-region="history"]',
  '.storybook-dock',
  'button.choice-row[data-action-id]',
  '.choice-bullet',
];

const FORBIDDEN_SELECTORS = ['.fake-tui', '.storybook-topline'];
const FORBIDDEN_VISIBLE_TEXT = ['CURRENT ENCOUNTER', 'LOCAL STATUS'];
const REQUIRED_WASM_RESOURCES = ['assets/wasm-pkg/escape_wasm.js', 'assets/wasm-pkg/escape_wasm_bg.wasm'];

const options = parseArgs(process.argv.slice(2));
const report = {
  baseUrl: options.baseUrl,
  requireWasm: options.requireWasm,
  generatedAt: new Date().toISOString(),
  viewports: [],
};

await mkdir(options.outDir, { recursive: true });
const screenshotsDir = path.join(options.outDir, 'screenshots');
await mkdir(screenshotsDir, { recursive: true });

let hasFailure = false;
const browser = await chromium.launch();
try {
  for (const viewport of VIEWPORTS) {
    const entry = await runViewportQa(browser, viewport, screenshotsDir);
    report.viewports.push(entry);
    if (!entry.passed) hasFailure = true;
  }
} finally {
  await browser.close();
}

const reportPath = path.join(options.outDir, 'visual-qa-report.json');
await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`, 'utf-8');

if (hasFailure) {
  console.error(`Web Storybook visual QA failed. Report: ${reportPath}`);
  process.exitCode = 1;
} else {
  console.log(`Web Storybook visual QA passed. Report: ${reportPath}`);
}

async function runViewportQa(browserInstance, viewport, screenshotsDirPath) {
  const checks = [];
  const context = await newFreshContext(browserInstance, viewport);
  const page = await context.newPage();
  const wasmResourcePromises = options.requireWasm ? waitForWasmResources(page) : [];
  let shellRect = null;
  let scrollWidth = null;
  let screenshot = null;

  try {
    await loadStorybookPage(page, wasmResourcePromises, checks);

    for (const selector of REQUIRED_SELECTORS) {
      record(checks, `required selector ${selector}`, (await page.locator(selector).count()) > 0);
    }
    for (const selector of FORBIDDEN_SELECTORS) {
      record(checks, `forbidden selector ${selector}`, (await page.locator(selector).count()) === 0);
    }

    const metrics = await collectLayoutMetrics(page);
    shellRect = metrics.shellRect;
    scrollWidth = metrics.scrollWidth;

    for (const text of FORBIDDEN_VISIBLE_TEXT) {
      record(checks, `forbidden visible text ${text}`, !metrics.visibleText.includes(text));
    }
    record(checks, 'browser title excludes legacy fake TUI wording', !/fake\s*tui/i.test(metrics.title));
    record(checks, 'documentElement.scrollWidth <= viewport width', metrics.scrollWidth.documentElement <= viewport.width, {
      documentElement: metrics.scrollWidth.documentElement,
      viewport: viewport.width,
    });
    record(checks, 'body.scrollWidth <= viewport width', metrics.scrollWidth.body <= viewport.width, {
      body: metrics.scrollWidth.body,
      viewport: viewport.width,
    });
    record(checks, 'storybook shell does not exceed viewport width', metrics.shellRect.width <= viewport.width + 1, {
      shellWidth: metrics.shellRect.width,
      viewport: viewport.width,
    });
    if (viewport.width >= 1000) {
      record(checks, 'wide desktop shell width stays in portrait-board range', metrics.shellRect.width >= 760 && metrics.shellRect.width <= 850, {
        shellWidth: metrics.shellRect.width,
      });
      record(checks, 'wide desktop shell is centered', Math.abs(metrics.leftMargin - metrics.rightMargin) <= 4, {
        leftMargin: metrics.leftMargin,
        rightMargin: metrics.rightMargin,
      });
    }

    const screenshotPath = path.join(screenshotsDirPath, `${viewport.name}.png`);
    await page.screenshot({ path: screenshotPath, fullPage: true });
    screenshot = path.relative(options.outDir, screenshotPath);

    await verifyInteractionChangesPage(browserInstance, viewport, 'click', checks);
    await verifyInteractionChangesPage(browserInstance, viewport, 'keyboard', checks);
  } catch (error) {
    record(checks, 'viewport QA completed without fatal error', false, { error: errorMessage(error) });
  } finally {
    await context.close();
  }

  return {
    name: viewport.name,
    width: viewport.width,
    height: viewport.height,
    passed: checks.every((check) => check.passed),
    checks,
    screenshot,
    shellRect,
    scrollWidth,
  };
}

async function verifyInteractionChangesPage(browserInstance, viewport, mode, parentChecks) {
  const context = await newFreshContext(browserInstance, viewport);
  const page = await context.newPage();
  const wasmResourcePromises = options.requireWasm ? waitForWasmResources(page) : [];
  const checkName = mode === 'click' ? 'first choice click changes rendered page' : 'number key 1 changes rendered page';

  try {
    await loadStorybookPage(page, wasmResourcePromises, parentChecks, { recordWasmChecks: false });
    const action = page.locator('button.choice-row[data-action-id]').first();
    await action.waitFor({ state: 'visible', timeout: 5_000 });
    const before = await shellFingerprint(page);
    if (mode === 'click') {
      await action.click();
    } else {
      await page.keyboard.press('1');
    }
    await page.waitForFunction((previous) => {
      const shell = document.querySelector('.storybook-shell');
      return Boolean(shell && `${shell.getAttribute('data-mode') ?? ''}\n${shell.textContent ?? ''}` !== previous);
    }, before, { timeout: 5_000 });
    const after = await shellFingerprint(page);
    record(parentChecks, checkName, before !== after);
  } catch (error) {
    record(parentChecks, checkName, false, { error: errorMessage(error) });
  } finally {
    await context.close();
  }
}

async function newFreshContext(browserInstance, viewport) {
  const context = await browserInstance.newContext({
    viewport: { width: viewport.width, height: viewport.height },
    reducedMotion: 'reduce',
    deviceScaleFactor: 1,
  });
  await context.addInitScript(() => {
    localStorage.clear();
    sessionStorage.clear();
  });
  return context;
}

async function loadStorybookPage(page, wasmResourcePromises, checks, { recordWasmChecks = true } = {}) {
  await page.goto(options.baseUrl, { waitUntil: 'domcontentloaded' });
  await page.waitForSelector('[data-renderer="web-storybook"]', { timeout: 10_000 });
  await page.evaluate(() => (document.fonts?.ready ? document.fonts.ready.then(() => true) : true));
  await page.waitForLoadState('networkidle', { timeout: 10_000 }).catch(() => undefined);

  if (options.requireWasm) {
    const wasmResponses = await Promise.all(wasmResourcePromises);
    if (recordWasmChecks) {
      for (const response of wasmResponses) {
        record(checks, `loaded ${response.resource}`, response.ok, response.details);
      }
    }
  }

  await page.waitForTimeout(100);
  if (options.requireWasm) {
    const warningCount = await page.locator('.storybook-runtime-warning').count();
    if (recordWasmChecks) {
      record(checks, 'WASM runtime warning is absent', warningCount === 0);
    }
  }
}

function waitForWasmResources(page) {
  return REQUIRED_WASM_RESOURCES.map((resource) =>
    page
      .waitForResponse((response) => response.url().includes(resource) && response.status() < 400, { timeout: 15_000 })
      .then((response) => ({
        resource,
        ok: true,
        details: { url: response.url(), status: response.status() },
      }))
      .catch((error) => ({
        resource,
        ok: false,
        details: { error: errorMessage(error) },
      })),
  );
}

async function collectLayoutMetrics(page) {
  return page.evaluate(() => {
    const shell = document.querySelector('.storybook-shell');
    if (!shell) throw new Error('missing .storybook-shell');
    const rect = shell.getBoundingClientRect();
    return {
      title: document.title,
      visibleText: document.body.innerText,
      shellRect: {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        left: rect.left,
      },
      leftMargin: rect.left,
      rightMargin: window.innerWidth - rect.right,
      scrollWidth: {
        documentElement: document.documentElement.scrollWidth,
        body: document.body.scrollWidth,
      },
    };
  });
}

async function shellFingerprint(page) {
  return page.evaluate(() => {
    const shell = document.querySelector('.storybook-shell');
    return shell ? `${shell.getAttribute('data-mode') ?? ''}\n${shell.textContent ?? ''}` : '';
  });
}

function parseArgs(argv) {
  const parsed = {
    baseUrl: undefined,
    outDir: undefined,
    requireWasm: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--help' || arg === '-h') {
      printUsage();
      process.exit(0);
    }
    if (arg === '--require-wasm') {
      parsed.requireWasm = true;
      continue;
    }
    if (arg === '--base-url' || arg === '--out-dir') {
      const value = argv[index + 1];
      if (!value || value.startsWith('--')) failUsage(`${arg} requires a value`);
      if (arg === '--base-url') parsed.baseUrl = value;
      if (arg === '--out-dir') parsed.outDir = path.resolve(value);
      index += 1;
      continue;
    }
    failUsage(`unknown argument: ${arg}`);
  }

  if (!parsed.baseUrl) failUsage('--base-url is required');
  if (!parsed.outDir) failUsage('--out-dir is required');
  try {
    const parsedUrl = new URL(parsed.baseUrl);
    if (!['http:', 'https:'].includes(parsedUrl.protocol)) failUsage('--base-url must be http(s)');
    parsed.baseUrl = parsedUrl.toString();
  } catch (error) {
    failUsage(`invalid --base-url: ${errorMessage(error)}`);
  }
  return parsed;
}

function record(checks, name, passed, details = {}) {
  checks.push({ name, passed: Boolean(passed), details });
}

function failUsage(message) {
  console.error(`error: ${message}`);
  printUsage();
  process.exit(2);
}

function printUsage() {
  console.log(`Usage: node scripts/storybook-reference-qa.mjs --base-url URL --out-dir DIR [--require-wasm]

Runs structural visual QA for the Web Storybook mobile pixel board.

Required:
  --base-url URL    Already-running Vite preview/player URL to test.
  --out-dir DIR     Scratch output directory for screenshots and visual-qa-report.json.

Optional:
  --require-wasm    Require Rust/WASM-primary resources and no runtime warning.`);
}

function errorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}
