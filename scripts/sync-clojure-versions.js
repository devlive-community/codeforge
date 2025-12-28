#!/usr/bin/env node

/**
 * 同步 Clojure 版本到阿里云 OSS
 *
 * 功能：
 * 1. 从 GitHub API 获取 Clojure 所有版本
 * 2. 下载版本文件
 * 3. 使用阿里云官方 ali-oss SDK 上传到 OSS /global/plugins/clojure/ 目录
 * 4. 生成 metadata.json 文件
 *
 * 使用方法：
 * node scripts/sync-clojure-versions.js
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
      // 跳过注释和空行
      if (!line || line.startsWith('#')) return;

      const match = line.match(/^([^=]+)=(.*)$/);
      if (match) {
        const key = match[1].trim();
        let value = match[2].trim();
        // 移除引号
        value = value.replace(/^["']|["']$/g, '');
        // 只在环境变量不存在时设置
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

// 获取配置（在 loadEnv 之后调用）
function getConfig() {
  return {
    ossRegion: process.env.OSS_REGION || 'oss-cn-hangzhou',
    ossAccessKeyId: process.env.OSS_ACCESS_KEY_ID,
    ossAccessKeySecret: process.env.OSS_ACCESS_KEY_SECRET,
    ossBucket: process.env.OSS_BUCKET,
    cdnDomain: process.env.CDN_DOMAIN, // 自定义 CDN 域名（可选）
    githubToken: process.env.GITHUB_TOKEN, // GitHub Personal Access Token（可选，用于提高 API 速率限制）
    githubRepo: 'clojure/brew-install',
    ossPrefix: 'global/plugins/clojure/',
    tempDir: path.join(__dirname, '.temp-clojure'),
    // 支持的平台
    platforms: ['macos-aarch64', 'macos-x86_64', 'linux-aarch64', 'linux-x86_64', 'windows-x86_64']
  };
}

// 全局配置变量
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

  // 显示配置信息（隐藏敏感信息）
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

    // 如果是 GitHub API 请求且配置了 Token，添加认证头
    if (url.includes('api.github.com') && CONFIG.githubToken) {
      options.headers['Authorization'] = `token ${CONFIG.githubToken}`;
    }

    client.get(url, options, (res) => {
      if (res.statusCode === 302 || res.statusCode === 301) {
        // 处理重定向
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
  console.log('正在获取 Clojure 版本列表...');
  const url = `https://api.github.com/repos/${CONFIG.githubRepo}/releases?per_page=100`;
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
    console.log('=== Clojure 版本同步工具 ===\n');

    // 加载 .env 文件
    loadEnv();

    // 初始化配置
    CONFIG = getConfig();

    // 验证配置
    validateConfig();

    // 创建 OSS 客户端
    const ossClient = createOSSClient();

    // 创建临时目录
    ensureTempDir();

    // 获取 releases
    const releases = await getGitHubReleases();
    console.log(`找到 ${releases.length} 个版本\n`);

    const metadata = {
      language: 'clojure',
      last_updated: new Date().toISOString(),
      releases: []
    };

    // 处理每个版本
    for (const release of releases) {
      const version = release.tag_name;
      console.log(`处理版本: ${version}`);

      // 查找 tar.gz 资源
      const asset = release.assets.find(a =>
        a.name.endsWith('.tar.gz') && a.name.includes('clojure-tools')
      );

      if (!asset) {
        console.log(`  ⚠ 跳过: 未找到 tar.gz 文件`);
        continue;
      }

      try {
        // 下载文件
        const fileName = asset.name;
        const localPath = path.join(CONFIG.tempDir, fileName);
        await downloadFile(asset.browser_download_url, localPath);

        // 计算文件信息
        const fileSize = getFileSize(localPath);
        const md5 = calculateMD5(localPath);

        // 上传到 OSS
        const ossPath = `${CONFIG.ossPrefix}${fileName}`;
        await uploadToOSS(ossClient, localPath, ossPath);

        // 添加到 metadata
        // 使用自定义 CDN 域名或默认 OSS 域名
        const cdnUrl = CONFIG.cdnDomain
          ? `${CONFIG.cdnDomain}/${ossPath}`
          : `https://${CONFIG.ossBucket}.${CONFIG.ossRegion}.aliyuncs.com/${ossPath}`;

        metadata.releases.push({
          version: version.replace(/^v/, ''),
          display_name: `Clojure ${version}`,
          published_at: release.published_at,
          download_url: cdnUrl,
          github_url: asset.browser_download_url,
          file_name: fileName,
          size: fileSize,
          md5: md5,
          supported_platforms: CONFIG.platforms
        });

        console.log(`  ✓ 版本 ${version} 处理完成\n`);

        // 删除本地文件
        fs.unlinkSync(localPath);
      } catch (error) {
        console.error(`  ✗ 处理版本 ${version} 失败: ${error.message}\n`);
      }
    }

    // 按版本号排序（最新的在前）
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

    // 输出 metadata URL
    const metadataUrl = CONFIG.cdnDomain
      ? `${CONFIG.cdnDomain}/${CONFIG.ossPrefix}metadata.json`
      : `https://${CONFIG.ossBucket}.${CONFIG.ossRegion}.aliyuncs.com/${CONFIG.ossPrefix}metadata.json`;
    console.log(`\nmetadata URL: ${metadataUrl}`);

  } catch (error) {
    console.error('\n✗ 错误:', error.message);
    process.exit(1);
  } finally {
    // 清理临时目录
    cleanupTempDir();
  }
}

// 运行
main();
