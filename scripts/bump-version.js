#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import {fileURLToPath} from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const newVersion = process.argv[2];
if (!newVersion) {
    console.error('请提供新版本号: node scripts/bump-version.js 25.0.1');
    process.exit(1);
}

// 更新 package.json
const packagePath = path.join(__dirname, '../package.json');
const packageJson = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
packageJson.version = newVersion;
fs.writeFileSync(packagePath, JSON.stringify(packageJson, null, 2) + '\n');

// 更新 Cargo.toml
const cargoPath = path.join(__dirname, '../src-tauri/Cargo.toml');
let cargoContent = fs.readFileSync(cargoPath, 'utf8');
cargoContent = cargoContent.replace(/^version = ".*"$/m, `version = "${newVersion}"`);
fs.writeFileSync(cargoPath, cargoContent);

// 更新 tauri.conf.json
const tauriConfigPath = path.join(__dirname, '../src-tauri/tauri.conf.json');
const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf8'));
tauriConfig.version = newVersion;
fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2) + '\n');

console.log(`✅ 版本已更新为 ${newVersion}`);
