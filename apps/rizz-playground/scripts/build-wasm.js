#!/usr/bin/env node
/**
 * Build script for RizzScript WASM module
 * Builds the WASM module and copies it to the playground's public folder
 */

import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const SCRIPT_DIR = __dirname;
const PLAYGROUND_DIR = path.dirname(SCRIPT_DIR);
const WASM_DIR = path.join(PLAYGROUND_DIR, '..', 'rizz-wasm');
const PUBLIC_DIR = path.join(PLAYGROUND_DIR, 'public');
const WASM_PUBLIC_DIR = path.join(PUBLIC_DIR, 'wasm');

console.log('🔨 Building RizzScript WASM module...');

// Check if wasm-pack is installed
try {
  execSync('wasm-pack --version', { stdio: 'ignore' });
} catch (error) {
  console.log('❌ wasm-pack is not installed. Installing...');
  execSync('curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh', {
    stdio: 'inherit',
    shell: '/bin/bash'
  });
}

// Build WASM module
console.log('📦 Building WASM package...');
process.chdir(WASM_DIR);

try {
  execSync('wasm-pack build --target web --release', { stdio: 'inherit' });
} catch (error) {
  console.error('\n❌ WASM build failed!');
  console.error('\nThis is expected because rizz-core currently depends on tokio,');
  console.error('which does not compile to WASM. The playground will use the');
  console.error('JavaScript fallback interpreter instead.\n');
  console.error('To enable WASM:');
  console.error('1. Refactor rizz-core with feature flags');
  console.error('2. Add "parser-only" feature without tokio');
  console.error('3. Update rizz-wasm to use parser-only feature');
  console.error('\nSee WASM_SETUP.md for details.\n');
  process.exit(1);
}

// Create wasm directory in public if it doesn't exist
if (!fs.existsSync(WASM_PUBLIC_DIR)) {
  fs.mkdirSync(WASM_PUBLIC_DIR, { recursive: true });
}

// Copy WASM files to public folder
console.log('📋 Copying WASM files to public folder...');
const pkgDir = path.join(WASM_DIR, 'pkg');
const files = fs.readdirSync(pkgDir);

files.forEach(file => {
  const src = path.join(pkgDir, file);
  const dest = path.join(WASM_PUBLIC_DIR, file);
  
  const stat = fs.statSync(src);
  if (stat.isFile()) {
    fs.copyFileSync(src, dest);
  } else if (stat.isDirectory()) {
    // Recursively copy directories
    if (!fs.existsSync(dest)) {
      fs.mkdirSync(dest, { recursive: true });
    }
    const subFiles = fs.readdirSync(src);
    subFiles.forEach(subFile => {
      fs.copyFileSync(
        path.join(src, subFile),
        path.join(dest, subFile)
      );
    });
  }
});

console.log('✅ WASM build complete!');
console.log(`📁 Files copied to: ${WASM_PUBLIC_DIR}`);
console.log('');
console.log('Files:');
const copiedFiles = fs.readdirSync(WASM_PUBLIC_DIR);
copiedFiles.forEach(file => {
  const filePath = path.join(WASM_PUBLIC_DIR, file);
  const stat = fs.statSync(filePath);
  if (stat.isFile()) {
    const size = (stat.size / 1024).toFixed(2);
    console.log(`  ${file} (${size} KB)`);
  }
});
