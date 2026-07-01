<template>
  <section class="max-w-5xl mx-auto px-5 py-20">
    <!-- 标题区域 -->
    <div class="text-center mb-16">
      <h1 class="text-5xl font-extrabold tracking-tight text-slate-900 dark:text-white mb-4">
        {{ t('download.title') }}
      </h1>
      <p v-if="latestRelease" class="text-lg text-slate-600 dark:text-slate-400">
        {{ t('download.latestPrefix') }}
        <span class="font-semibold text-brand-600 dark:text-brand-400">v{{ latestRelease.version }}</span>
        {{ t('download.latestSuffix') }}
        <RouterLink class="font-medium text-brand-600 dark:text-brand-400 hover:underline underline-offset-2" 
                    :to="`/release/${latestRelease.version}`">
          {{ t('download.releaseNotes') }}
        </RouterLink>
      </p>
    </div>

    <!-- 版本列表 -->
    <div class="space-y-6">
      <div v-for="version in releases" :key="version.version"
           class="rounded-2xl border-2 border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900/50 overflow-hidden">
        <!-- 版本头部 -->
        <div class="flex items-center justify-between px-6 py-4 bg-slate-50 dark:bg-slate-900/80 border-b border-slate-200 dark:border-slate-800">
          <div class="flex items-center gap-4">
            <span class="text-xl font-bold text-slate-900 dark:text-white">v{{ version.version }}</span>
            <span v-if="version.date" class="text-sm text-slate-500 dark:text-slate-400">{{ version.date }}</span>
            <span v-if="version.version === latestRelease?.version" 
                  class="px-2.5 py-0.5 rounded-full bg-brand-500 text-white text-xs font-semibold">
              最新
            </span>
          </div>
          <RouterLink :to="`/release/${version.version}`"
                      class="text-sm text-brand-600 dark:text-brand-400 hover:underline">
            查看详情 →
          </RouterLink>
        </div>
        
        <!-- 下载链接 -->
        <div class="grid sm:grid-cols-2 gap-3 p-6">
          <!-- Windows -->
          <div class="space-y-2">
            <div class="flex items-center gap-2 text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">
              <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
                <path d="M3 5.6 10.2 4.6v6.7H3zM10.2 12v6.7L3 17.7V12zM11.1 4.5 21 3v8.3h-9.9zM21 12.7V21l-9.9-1.5V12.7z"/>
              </svg>
              <span>Windows</span>
            </div>
            <a :href="getDownloadUrl(version.version, 'exe')" 
               class="group flex items-center justify-between px-4 py-3 rounded-xl border border-slate-200 dark:border-slate-800 hover:border-brand-400 dark:hover:border-brand-500 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-all">
              <div class="flex items-center gap-3">
                <svg class="w-5 h-5 text-slate-400 group-hover:text-brand-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>
                </svg>
                <span class="text-sm font-medium text-slate-700 dark:text-slate-300">.exe 安装包</span>
              </div>
              <span class="text-xs text-slate-400">x64</span>
            </a>
            <a :href="getDownloadUrl(version.version, 'msi')" 
               class="group flex items-center justify-between px-4 py-3 rounded-xl border border-slate-200 dark:border-slate-800 hover:border-brand-400 dark:hover:border-brand-500 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-all">
              <div class="flex items-center gap-3">
                <svg class="w-5 h-5 text-slate-400 group-hover:text-brand-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>
                </svg>
                <span class="text-sm font-medium text-slate-700 dark:text-slate-300">.msi 安装包</span>
              </div>
              <span class="text-xs text-slate-400">x64</span>
            </a>
          </div>
          
          <!-- macOS -->
          <div class="space-y-2">
            <div class="flex items-center gap-2 text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">
              <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
                <path d="M16.3 1.4c.1 1-.3 2-.9 2.8-.7.8-1.7 1.4-2.7 1.3-.1-1 .4-2 .9-2.7.7-.8 1.8-1.4 2.7-1.4zM19 17.2c-.5 1.1-.7 1.6-1.3 2.6-.9 1.4-2.1 3.1-3.7 3.1-1.4 0-1.7-.9-3.6-.9s-2.3.9-3.6.9c-1.5 0-2.7-1.5-3.6-2.9C.7 16.3.4 12 2 9.5c.9-1.4 2.5-2.3 4-2.3 1.5 0 2.5 1 3.7 1 1.2 0 1.9-1 3.7-1 1.3 0 2.7.7 3.7 2-3.2 1.8-2.7 6.4.2 8z"/>
              </svg>
              <span>macOS</span>
            </div>
            <a :href="getDownloadUrl(version.version, 'dmg')" 
               class="group flex items-center justify-between px-4 py-3 rounded-xl border border-slate-200 dark:border-slate-800 hover:border-brand-400 dark:hover:border-brand-500 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-all">
              <div class="flex items-center gap-3">
                <svg class="w-5 h-5 text-slate-400 group-hover:text-brand-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>
                </svg>
                <span class="text-sm font-medium text-slate-700 dark:text-slate-300">.dmg 安装包</span>
              </div>
              <span class="text-xs text-slate-400">Universal</span>
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- GitHub 链接 -->
    <div class="mt-10 text-center">
      <a href="https://github.com/devlive-community/codeforge/releases" target="_blank" rel="noopener"
         class="inline-flex items-center gap-2 px-5 py-3 rounded-xl text-slate-600 dark:text-slate-400 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all group">
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2C6.48 2 2 6.58 2 12.26c0 4.5 2.87 8.32 6.84 9.67.5.1.68-.22.68-.49 0-.24-.01-.87-.01-1.71-2.78.62-3.37-1.37-3.37-1.37-.45-1.18-1.11-1.5-1.11-1.5-.91-.64.07-.62.07-.62 1 .07 1.53 1.06 1.53 1.06.89 1.56 2.34 1.11 2.91.85.09-.66.35-1.11.63-1.37-2.22-.26-4.56-1.14-4.56-5.07 0-1.12.39-2.03 1.03-2.75-.1-.26-.45-1.3.1-2.71 0 0 .84-.27 2.75 1.05A9.36 9.36 0 0 1 12 7.07c.85 0 1.71.12 2.51.34 1.91-1.32 2.75-1.05 2.75-1.05.55 1.41.2 2.45.1 2.71.64.72 1.03 1.63 1.03 2.75 0 3.94-2.34 4.81-4.57 5.06.36.32.68.94.68 1.9 0 1.37-.01 2.48-.01 2.82 0 .27.18.6.69.49A10.02 10.02 0 0 0 22 12.26C22 6.58 17.52 2 12 2z"/>
        </svg>
        <span class="font-medium">在 GitHub 查看所有版本</span>
        <span class="group-hover:translate-x-1 transition-transform">→</span>
      </a>
    </div>
  </section>
</template>

<script setup lang="ts">
import {useI18n} from 'vue-i18n'
import {releases, latestRelease} from '../content/releases'

const {t} = useI18n()

const getDownloadUrl = (version: string, type: 'exe' | 'msi' | 'dmg') => {
  const baseUrl = `https://github.com/devlive-community/codeforge/releases/download/${version}`
  switch (type) {
    case 'exe':
      return `${baseUrl}/CodeForge_${version}_x64-setup.exe`
    case 'msi':
      return `${baseUrl}/CodeForge_${version}_x64_en-US.msi`
    case 'dmg':
      return `${baseUrl}/CodeForge_${version}_universal.dmg`
  }
}
</script>
