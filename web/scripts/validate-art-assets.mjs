#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const manifest = path.join(root, 'web/src/ui/storybook/art/artManifest.ts');
const assets = path.join(root, 'web/public/assets/art');
const maxBytes = 150 * 1024;
const allow = new Set(['location_jianghu_market_street.webp', 'location_jianghu_roadside.webp']);

function error(id, file, rule, detail) {
  console.error('art-assets: ' + id + ' -> ' + file + ': rule=' + rule + (detail ? ' (' + detail + ')' : ''));
  process.exitCode = 1;
}
function dimensions(buf) {
  if (buf.toString('ascii', 0, 4) !== 'RIFF' || buf.toString('ascii', 8, 12) !== 'WEBP') return null;
  const kind = buf.toString('ascii', 12, 16);
  if (kind === 'VP8X' && buf.length >= 30) return [1 + buf[24] + (buf[25] << 8) + (buf[26] << 16), 1 + buf[27] + (buf[28] << 8) + (buf[29] << 16)];
  if (kind === 'VP8L' && buf[20] === 0x2f) { const x = buf[21] | buf[22] << 8 | buf[23] << 16 | buf[24] << 24; return [1 + (x & 0x3fff), 1 + ((x >>> 14) & 0x3fff)]; }
  if (kind === 'VP8 ' && buf[23] === 0x9d && buf[24] === 1 && buf[25] === 0x2a) return [buf.readUInt16LE(26) & 0x3fff, buf.readUInt16LE(28) & 0x3fff];
  return null;
}
const source = fs.readFileSync(manifest, 'utf8');
const body = source.slice(source.indexOf('{', source.indexOf('artManifest')), source.indexOf('};', source.indexOf('artManifest'))).split('\n').filter((line) => !line.trim().startsWith('//')).join('\n');
const entries = [...body.matchAll(/['"]([^'"]+)['"]\s*:\s*['"]([^'"]+)['"]/g)].map(([, id, file]) => ({ id, file })).filter(({ file }) => file !== 'title_hero.webp');
if (!entries.length) error('manifest', '-', 'nonzero in-scope mappings');
for (const { id, file } of entries) {
  if (path.basename(file) !== file || file.includes('..') || file.includes('\\')) { error(id, file, 'safe asset path'); continue; }
  const target = path.join(assets, file);
  if (!fs.existsSync(target)) { error(id, file, 'asset exists', target); continue; }
  const size = fs.statSync(target).size;
  if (!file.toLowerCase().endsWith('.webp')) error(id, file, 'WebP extension');
  if (size > maxBytes) error(id, file, 'size <= 150KB', size + ' bytes');
  const dim = dimensions(fs.readFileSync(target));
  if (!dim) { error(id, file, 'valid WebP dimensions'); continue; }
  const delta = Math.abs(dim[0] * 3 - dim[1] * 5);
  if (delta > 1 && !allow.has(file)) error(id, file, '5:3 aspect', dim[0] + 'x' + dim[1] + ', delta ' + delta);
  else if (delta > 1) console.warn('art-assets: ' + id + ' -> ' + file + ': temporary aspect allowlist (' + dim[0] + 'x' + dim[1] + ')');
}
if (!process.exitCode) console.log('art-assets: validated ' + entries.length + ' in-scope mappings (title_hero excluded)');
