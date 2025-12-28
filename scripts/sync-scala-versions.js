#!/usr/bin/env node

/**
 * 同步 Scala 版本到阿里云 OSS
 *
 * 功能：
 * 1. 从 GitHub API 获取 Scala 所有版本
 * 2. 下载版本文件
 * 3. 使用阿里云官方 ali-oss SDK 上传到 OSS /global/plugins/scala/ 目录
 * 4. 生成 metadata.json 文件
 *
 * 使用方法：
 * node scripts/sync-scala-versions.js
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

// ES 模块中获取 __dirname
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 加载 .env 文件
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

// 获取配置
function getConfig() {
  return {
    ossRegion: process.env.OSS_REGION || 'oss-cn-hangzhou',
    ossAccessKeyId: process.env.OSS_ACCESS_KEY_ID,
    ossAccessKeySecret: process.env.OSS_ACCESS_KEY_SECRET,
    ossBucket: process.env.OSS_BUCKET,
    cdnDomain: process.env.CDN_DOMAIN,
    githubRepo: 'lampepfl/dotty',
    ossPrefix: 'global/plugins/scala/',
    tempDir: path.join(__dirname, '.temp-scala'),
    platformMap: {
      'aarch64-apple-darwin': 'macos-aarch64',
      'x86_64-apple-darwin': 'macos-x86_64',
      'aarch64-pc-linux': 'linux-aarch64',
      'x86_64-pc-linux': 'linux-x86_64',
      'x86_64-pc-win32': 'windows-x86_64'
    }
  };
}

let CONFIG;

// 验证配置
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

// 创建临时目录
function ensureTempDir() {
  if (!fs.existsSync(CONFIG.tempDir)) {
    fs.mkdirSync(CONFIG.tempDir, { recursive: true });
  }
}

// 清理临时目录
function cleanupTempDir() {
  if (fs.existsSync(CONFIG.tempDir)) {
    fs.rmSync(CONFIG.tempDir, { recursive: true, force: true });
  }
}

// HTTP(S) GET 请求
function httpGet(url, isJson = true) {
  return new Promise((resolve, reject) => {
    const client = url.startsWith('https') ? https : http;
    const options = {
      headers: {
        'User-Agent': 'CodeForge-Sync-Script'
      }
    };

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

// 下载文件
async function downloadFile(url, destPath) {
  console.log(`  下载: ${url}`);
  const data = await httpGet(url, false);
  fs.writeFileSync(destPath, data);
  return destPath;
}

// 获取 GitHub releases
async function getGitHubReleases() {
  console.log('正在获取 Scala 版本列表...');
  const url = `https://api.github.com/repos/${CONFIG.githubRepo}/releases?per_page=10`;
  return await httpGet(url);
}

// 计算文件 MD5
function calculateMD5(filePath) {
  const buffer = fs.readFileSync(filePath);
  return crypto.createHash('md5').update(buffer).digest('hex');
}

// 获取文件大小
function getFileSize(filePath) {
  const stats = fs.statSync(filePath);
  return stats.size;
}

// 创建 OSS 客户端
function createOSSClient() {
  return new OSS({
    region: CONFIG.ossRegion,
    accessKeyId: CONFIG.ossAccessKeyId,
    accessKeySecret: CONFIG.ossAccessKeySecret,
    bucket: CONFIG.ossBucket
  });
}

// 上传文件到 OSS
async function uploadToOSS(client, localPath, ossPath) {
  try {
    await client.put(ossPath, localPath);
    console.log(`  ✓ 上传成功: ${ossPath}`);
  } catch (error) {
    throw new Error(`上传失败: ${error.message}`);
  }
}

// 上传 metadata.json 到 OSS
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

// 主函数
async function main() {
  try {
    console.log('=== Scala 版本同步工具 ===\n');

    loadEnv();

    CONFIG = getConfig();

    validateConfig();

    const ossClient = createOSSClient();

    ensureTempDir();

    const releases = await getGitHubReleases();
    console.log(`找到 ${releases.length} 个版本\n`);

    const metadata = {
      language: 'scala',
      last_updated: new Date().toISOString(),
      releases: []
    };

    for (const release of releases) {
      const version = release.tag_name;
      console.log(`处理版本: ${version}`);

      const scalaAssets = release.assets.filter(a =>
        a.name.endsWith('.tar.gz') && a.name.includes('scala3')
      );

      if (scalaAssets.length === 0) {
        console.log(`  ⚠ 跳过: 未找到 tar.gz 文件`);
        continue;
      }

      const versionClean = version.replace(/^v/, '');
      const platformAssets = {};

      for (const asset of scalaAssets) {
        const platformMatch = asset.name.match(/scala3-[\d.]+-(?:RC\d+-)?([^.]+)\.tar\.gz/);
        if (platformMatch) {
          const githubPlatform = platformMatch[1];
          const mappedPlatform = CONFIG.platformMap[githubPlatform];
          if (mappedPlatform && !platformAssets[mappedPlatform]) {
            platformAssets[mappedPlatform] = asset;
          }
        }
      }

      let processedCount = 0;

      for (const [platform, asset] of Object.entries(platformAssets)) {
        try {
          const fileName = asset.name;
          const localPath = path.join(CONFIG.tempDir, fileName);
          await downloadFile(asset.browser_download_url, localPath);

          const fileSize = getFileSize(localPath);
          const md5 = calculateMD5(localPath);

          const ossPath = `${CONFIG.ossPrefix}${versionClean}/${fileName}`;
          await uploadToOSS(ossClient, localPath, ossPath);

          const cdnUrl = CONFIG.cdnDomain
            ? `${CONFIG.cdnDomain}/${ossPath}`
            : `https://${CONFIG.ossBucket}.${CONFIG.ossRegion}.aliyuncs.com/${ossPath}`;

          metadata.releases.push({
            version: versionClean,
            display_name: `Scala ${version}`,
            published_at: release.published_at,
            download_url: cdnUrl,
            github_url: asset.browser_download_url,
            file_name: fileName,
            size: fileSize,
            md5: md5,
            supported_platforms: [platform]
          });

          processedCount++;
          fs.unlinkSync(localPath);
        } catch (error) {
          console.error(`  ✗ 处理文件 ${asset.name} 失败: ${error.message}`);
        }
      }

      if (processedCount > 0) {
        console.log(`  ✓ 版本 ${version} 处理完成 (${processedCount} 个平台)\n`);
      } else {
        console.log(`  ⚠ 版本 ${version} 没有可用的平台文件\n`);
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
