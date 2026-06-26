<template>
  <section class="max-w-3xl mx-auto px-5 py-20">
    <div class="text-center">
      <h1 class="text-4xl font-extrabold tracking-tight text-slate-900 dark:text-white">下载 CodeForge</h1>
      <p class="mt-4 text-slate-600 dark:text-slate-400">
        当前最新版本
        <img class="inline-block align-middle h-5" alt="latest release"
             src="https://img.shields.io/github/v/release/devlive-community/codeforge.svg"/>
        <template v-if="latestRelease">
          ，详见
          <RouterLink class="text-brand-600 dark:text-brand-400 hover:underline" :to="`/release/${latestRelease.version}`">发布说明</RouterLink>。
        </template>
      </p>
    </div>

    <div class="mt-12 grid sm:grid-cols-2 gap-4">
      <a v-for="p in platforms" :key="p.name"
         href="https://github.com/devlive-community/codeforge/releases/latest" target="_blank" rel="noopener"
         class="group flex items-center gap-4 p-5 rounded-2xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900/40 hover:border-brand-300 dark:hover:border-brand-600 hover:shadow-soft transition-all">
        <div class="w-11 h-11 rounded-xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-700 dark:text-slate-200" v-html="p.icon"></div>
        <div>
          <div class="font-semibold text-slate-900 dark:text-white">{{ p.name }}</div>
          <div class="text-sm text-slate-500 dark:text-slate-400">{{ p.note }}</div>
        </div>
        <span class="ml-auto text-slate-300 group-hover:text-brand-500 transition-colors">↓</span>
      </a>
    </div>

    <div class="mt-10 text-center">
      <a href="https://github.com/devlive-community/codeforge/releases" target="_blank" rel="noopener"
         class="inline-flex items-center gap-2 text-sm text-slate-500 dark:text-slate-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">
        查看全部历史版本与安装包
        <span>→</span>
      </a>
    </div>
  </section>
</template>

<script setup lang="ts">
import {latestRelease} from '../content/releases'

const win = `<svg class="w-6 h-6" viewBox="0 0 24 24" fill="currentColor"><path d="M3 5.6 10.2 4.6v6.7H3zM10.2 12v6.7L3 17.7V12zM11.1 4.5 21 3v8.3h-9.9zM21 12.7V21l-9.9-1.5V12.7z"/></svg>`
const mac = `<svg class="w-6 h-6" viewBox="0 0 24 24" fill="currentColor"><path d="M16.3 1.4c.1 1-.3 2-.9 2.8-.7.8-1.7 1.4-2.7 1.3-.1-1 .4-2 .9-2.7.7-.8 1.8-1.4 2.7-1.4zM19 17.2c-.5 1.1-.7 1.6-1.3 2.6-.9 1.4-2.1 3.1-3.7 3.1-1.4 0-1.7-.9-3.6-.9s-2.3.9-3.6.9c-1.5 0-2.7-1.5-3.6-2.9C.7 16.3.4 12 2 9.5c.9-1.4 2.5-2.3 4-2.3 1.5 0 2.5 1 3.7 1 1.2 0 1.9-1 3.7-1 1.3 0 2.7.7 3.7 2-3.2 1.8-2.7 6.4.2 8z"/></svg>`

const platforms = [
  {name: 'Windows', note: '.msi / .exe 安装包', icon: win},
  {name: 'macOS', note: 'Apple Silicon & Intel', icon: mac}
]
</script>
