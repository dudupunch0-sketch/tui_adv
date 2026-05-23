import { cp, lstat, mkdir, rm } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(scriptDir, '..');
const outDirArg = readOption('--out-dir');
const outDir = resolveSafeOutDir(outDirArg ?? 'dist');
const sourceDir = path.join(webRoot, 'src', 'core', 'wasm-pkg');
const targetDir = path.join(outDir, 'assets', 'wasm-pkg');

await assertNoSymlinkComponents(sourceDir, webRoot);
await assertFile(path.join(sourceDir, 'escape_wasm.js'));
await assertFile(path.join(sourceDir, 'escape_wasm_bg.wasm'));
await assertNoSymlinkComponents(path.dirname(targetDir), webRoot);
await mkdir(path.dirname(targetDir), { recursive: true });
await assertNoSymlinkComponents(targetDir, webRoot);
await assertExistingTargetDirectory(targetDir);
await rm(targetDir, { recursive: true, force: true });
await assertNoSymlinkComponents(path.dirname(targetDir), webRoot);
await cp(sourceDir, targetDir, { recursive: true });

console.log(`copied wasm package: ${path.relative(webRoot, sourceDir)} -> ${path.relative(webRoot, targetDir)}`);

function readOption(name) {
  const index = process.argv.indexOf(name);
  if (index === -1) return undefined;
  const value = process.argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${name} requires a value`);
  }
  return value;
}

function resolveSafeOutDir(value) {
  if (!value.trim()) {
    throw new Error('--out-dir must be a non-empty relative path inside web/');
  }
  if (path.isAbsolute(value)) {
    throw new Error('--out-dir must be relative and stay inside web/');
  }
  const parts = value.split(/[\\/]+/).filter(Boolean);
  if (parts.includes('..')) {
    throw new Error('--out-dir must not contain parent-directory traversal');
  }
  const resolved = path.resolve(webRoot, value);
  if (!isInside(webRoot, resolved)) {
    throw new Error('--out-dir must stay inside web/');
  }
  return resolved;
}

function isInside(root, candidate) {
  const relative = path.relative(root, candidate);
  return relative === '' || (!relative.startsWith('..') && !path.isAbsolute(relative));
}

async function assertNoSymlinkComponents(candidatePath, rootPath) {
  if (!isInside(rootPath, candidatePath)) {
    throw new Error(`Refusing to use path outside web/: ${candidatePath}`);
  }
  const relative = path.relative(rootPath, candidatePath);
  let current = rootPath;
  for (const part of relative.split(path.sep).filter(Boolean)) {
    current = path.join(current, part);
    let info;
    try {
      info = await lstat(current);
    } catch (error) {
      if (error?.code === 'ENOENT') continue;
      throw error;
    }
    if (info.isSymbolicLink()) {
      throw new Error(`Refusing to use symlink component: ${path.relative(rootPath, current)}`);
    }
  }
}

async function assertExistingTargetDirectory(targetPath) {
  let info;
  try {
    info = await lstat(targetPath);
  } catch (error) {
    if (error?.code === 'ENOENT') return;
    throw error;
  }
  if (info.isSymbolicLink()) {
    throw new Error(`Refusing to replace symlink target: ${path.relative(webRoot, targetPath)}`);
  }
  if (!info.isDirectory()) {
    throw new Error(`Refusing to replace non-directory target: ${path.relative(webRoot, targetPath)}`);
  }
}

async function assertFile(filePath) {
  try {
    const info = await lstat(filePath);
    if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${filePath} is not a regular file`);
  } catch (error) {
    throw new Error(
      `Missing generated wasm package file: ${filePath}. Run npm run wasm:build before copying.`,
      { cause: error },
    );
  }
}
