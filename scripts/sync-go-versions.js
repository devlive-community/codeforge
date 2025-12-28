#!/usr/bin/env node

/**
 * 同步 Go 版本到阿里云 OSS
 *
 * 功能：
 * 1. 从 GitHub API 获取 Go 所有版本
 * 2. 下载版本文件
 * 3. 使用阿里云官方 ali-oss SDK 上传到 OSS /global/plugins/go/ 目录
 * 4. 生成 metadata.json 文件
 *
 * 使用方法：
 * node scripts/sync-go-versions.js
 *
 * 环境变量（可以在 .env 文件中配置）：
 * - OSS_REGION: 阿里云 OSS 区域
 * - OSS_ACCESS_KEY_ID: 阿里云访问密钥 ID
 * - OSS_ACCESS_KEY_SECRET: 阿里云访问密钥 Secret
 * - OSS_BUCKET: OSS Bucket 名称
 * - CDN_DOMAIN: 自定义 CDN 域名（可选）
 */

import https from 'https';
import http from 'http';
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { fileURLToPath } from 'url';
import OSS from 'ali-oss';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function loadEnv() {
  const envPath = path.join(__dirname, '..', '.env');
  if (fs.existsSync(envPath)) {
    const envContent = fs.readFileSync(envPath, 'utf8');
    let loadedCount = 0;

    envContent.split('\n').forEach(line => {
      line = line.trim();
      if (!line || line.startsWith('#')) return;

      const match = line.match(/^([^=]+)=(.*)$/);
      if (match) {
        const key = match[1].trim();
        let value = match[2].trim();
        value = value.replace(/^["']|["']$/g, '');
        if (!process.env[key]) {
          process.env[key] = value;
          loadedCount++;
        }
      }
    });

    console.log(`✓ 已从 .env 文件加载 ${loadedCount} 个环境变量\n`);
  } else {
    console.log('⚠ 未找到 .env 文件，将使用环境变量或默认值\n');
  }
}

function getConfig() {
  return {
    ossRegion: process.env.OSS_REGION || 'oss-cn-hangzhou',
    ossAccessKeyId: process.env.OSS_ACCESS_KEY_ID,
    ossAccessKeySecret: process.env.OSS_ACCESS_KEY_SECRET,
    ossBucket: process.env.OSS_BUCKET,
    cdnDomain: process.env.CDN_DOMAIN,
    githubToken: process.env.GITHUB_TOKEN,
    githubRepo: 'golang/go',
    ossPrefix: 'global/plugins/go/',
    tempDir: path.join(__dirname, '.temp-go'),
    platformMap: {
      'darwin-arm64': 'macos-aarch64',
      'darwin-amd64': 'macos-x86_64',
      'linux-arm64': 'linux-aarch64',
      'linux-amd64': 'linux-x86_64',
      'windows-amd64': 'windows-x86_64'
    }
  };
}

let CONFIG;

function validateConfig() {
  const missing = [];
  if (!CONFIG.ossAccessKeyId) missing.push('OSS_ACCESS_KEY_ID');
  if (!CONFIG.ossAccessKeySecret) missing.push('OSS_ACCESS_KEY_SECRET');
  if (!CONFIG.ossBucket) missing.push('OSS_BUCKET');

  if (missing.length > 0) {
    console.error('错误: 请设置以下环境变量:');
    missing.forEach(key => console.error(`  - ${key}`));
    console.error('\n提示: 可以在 .env 文件中配置这些变量');
    console.error('示例: cp .env.example .env');
    process.exit(1);
  }

  console.log('配置信息:');
  console.log(`  OSS Region: ${CONFIG.ossRegion}`);
  console.log(`  OSS Bucket: ${CONFIG.ossBucket}`);
  console.log(`  CDN Domain: ${CONFIG.cdnDomain || '未配置 (使用默认 OSS 域名)'}`);
  console.log('');
}

function ensureTempDir() {
  if (!fs.existsSync(CONFIG.tempDir)) {
    fs.mkdirSync(CONFIG.tempDir, { recursive: true });
  }
}

function cleanupTempDir() {
  if (fs.existsSync(CONFIG.tempDir)) {
    fs.rmSync(CONFIG.tempDir, { recursive: true, force: true });
  }
}

function httpGet(url, isJson = true) {
  return new Promise((resolve, reject) => {
    const client = url.startsWith('https') ? https : http;
    const options = {
      headers: {
        'User-Agent': 'CodeForge-Sync-Script'
      }
    };

    if (url.includes('api.github.com') && CONFIG.githubToken) {
      options.headers['Authorization'] = `token ${CONFIG.githubToken}`;
    }

    client.get(url, options, (res) => {
      if (res.statusCode === 302 || res.statusCode === 301) {
        return httpGet(res.headers.location, isJson).then(resolve).catch(reject);
      }

      if (res.statusCode !== 200) {
        reject(new Error(`HTTP ${res.statusCode}: ${url}`));
        return;
      }

      const chunks = [];
      res.on('data', chunk => chunks.push(chunk));
      res.on('end', () => {
        const data = Buffer.concat(chunks);
        if (isJson) {
          try {
            resolve(JSON.parse(data.toString()));
          } catch (e) {
            reject(new Error(`JSON 解析失败: ${e.message}`));
          }
        } else {
          resolve(data);
        }
      });
    }).on('error', reject);
  });
}

async function downloadFile(url, destPath) {
  console.log(`  下载: ${url}`);
  const data = await httpGet(url, false);
  fs.writeFileSync(destPath, data);
  return destPath;
}

async function getGoReleases() {
  console.log('正在获取 Go 版本列表...');
  const url = 'https://go.dev/dl/?mode=json&include=all';
  return await httpGet(url);
}

function calculateMD5(filePath) {
  const buffer = fs.readFileSync(filePath);
  return crypto.createHash('md5').update(buffer).digest('hex');
}

function getFileSize(filePath) {
  const stats = fs.statSync(filePath);
  return stats.size;
}

function createOSSClient() {
  return new OSS({
    region: CONFIG.ossRegion,
    accessKeyId: CONFIG.ossAccessKeyId,
    accessKeySecret: CONFIG.ossAccessKeySecret,
    bucket: CONFIG.ossBucket
  });
}

async function uploadToOSS(client, localPath, ossPath) {
  try {
    await client.put(ossPath, localPath);
    console.log(`  ✓ 上传成功: ${ossPath}`);
  } catch (error) {
    throw new Error(`上传失败: ${error.message}`);
  }
}

async function uploadMetadata(client, metadata) {
  try {
    const metadataJson = JSON.stringify(metadata, null, 2);
    const buffer = Buffer.from(metadataJson, 'utf8');
    const ossPath = `${CONFIG.ossPrefix}metadata.json`;

    await client.put(ossPath, buffer);
    console.log(`✓ metadata.json 上传成功`);
  } catch (error) {
    throw new Error(`上传 metadata 失败: ${error.message}`);
  }
}

async function main() {
  try {
    console.log('=== Go 版本同步工具 ===\n');

    loadEnv();
    CONFIG = getConfig();
    validateConfig();

    const ossClient = createOSSClient();
    ensureTempDir();

    const releases = await getGoReleases();
    console.log(`找到 ${releases.length} 个版本\n`);

    const metadata = {
      language: 'go',
      last_updated: new Date().toISOString(),
      releases: []
    };

    const goOsMap = {
      'darwin': ['macos-aarch64', 'macos-x86_64'],
      'linux': ['linux-aarch64', 'linux-x86_64'],
      'windows': ['windows-x86_64']
    };

    const goArchMap = {
      'arm64': 'aarch64',
      'amd64': 'x86_64'
    };

    for (const release of releases) {
      const version = release.version.replace(/^go/, '');
      console.log(`处理版本: ${version}`);

      const archiveFiles = release.files.filter(f => f.kind === 'archive');

      if (archiveFiles.length === 0) {
        console.log(`  ⚠ 跳过: 未找到归档文件`);
        continue;
      }

      let processedCount = 0;

      for (const file of archiveFiles) {
        const mappedArch = goArchMap[file.arch] || file.arch;
        const osPlatforms = goOsMap[file.os];

        if (!osPlatforms) continue;

        const platform = osPlatforms.find(p => p.includes(mappedArch));
        if (!platform) continue;

        try {
          const fileName = file.filename;
          const localPath = path.join(CONFIG.tempDir, fileName);
          const goDevUrl = `https://go.dev/dl/${fileName}`;

          console.log(`  下载 ${fileName}...`);
          await downloadFile(goDevUrl, localPath);

          const fileSize = getFileSize(localPath);
          const md5 = calculateMD5(localPath);

          const ossPath = `${CONFIG.ossPrefix}${version}/${fileName}`;
          await uploadToOSS(ossClient, localPath, ossPath);

          const cdnUrl = CONFIG.cdnDomain
            ? `${CONFIG.cdnDomain}/${ossPath}`
            : `https://${CONFIG.ossBucket}.${CONFIG.ossRegion}.aliyuncs.com/${ossPath}`;

          metadata.releases.push({
            version: version,
            display_name: `Go ${release.version}`,
            published_at: new Date().toISOString(),
            download_url: cdnUrl,
            github_url: goDevUrl,
            file_name: fileName,
            size: fileSize,
            md5: md5,
            supported_platforms: [platform]
          });

          processedCount++;
          fs.unlinkSync(localPath);
        } catch (error) {
          console.error(`  ✗ 处理文件 ${file.filename} 失败: ${error.message}`);
        }
      }

      if (processedCount > 0) {
        console.log(`  ✓ 版本 ${release.version} 处理完成 (${processedCount} 个平台)\n`);
      } else {
        console.log(`  ⚠ 版本 ${release.version} 没有可用的平台文件\n`);
      }
    }

    metadata.releases.sort((a, b) => {
      const versionA = a.version.split('.').map(Number);
      const versionB = b.version.split('.').map(Number);
      for (let i = 0; i < Math.max(versionA.length, versionB.length); i++) {
        const numA = versionA[i] || 0;
        const numB = versionB[i] || 0;
        if (numA !== numB) return numB - numA;
      }
      return 0;
    });

    console.log('\n上传 metadata.json...');
    await uploadMetadata(ossClient, metadata);

    console.log(`\n✓ 同步完成！共处理 ${metadata.releases.length} 个版本`);

    const metadataUrl = CONFIG.cdnDomain
      ? `${CONFIG.cdnDomain}/${CONFIG.ossPrefix}metadata.json`
      : `https://${CONFIG.ossBucket}.${CONFIG.ossRegion}.aliyuncs.com/${CONFIG.ossPrefix}metadata.json`;
    console.log(`\nmetadata URL: ${metadataUrl}`);

  } catch (error) {
    console.error('\n✗ 错误:', error.message);
    process.exit(1);
  } finally {
    cleanupTempDir();
  }
}

main();
