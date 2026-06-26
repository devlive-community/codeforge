<template>
  <section class="max-w-3xl mx-auto px-4 py-16 text-center">
    <h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">下载 CodeForge</h1>
    <p class="mt-4 text-gray-600 dark:text-gray-400">
      当前最新版本
      <img class="inline-block align-middle" alt="latest release"
           src="https://img.shields.io/github/v/release/devlive-community/codeforge.svg"/>
      <template v-if="latestRelease">
        ，详见 <RouterLink class="text-brand-600 dark:text-brand-400 hover:underline" :to="`/release/${latestRelease.version}`">发布说明</RouterLink>。
      </template>
    </p>

    <div class="mt-10 grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div v-for="p in platforms" :key="p.name"
           class="p-6 rounded-2xl border border-gray-200 dark:border-gray-800">
        <div class="text-3xl mb-2">{{ p.icon }}</div>
        <div class="font-semibold text-gray-900 dark:text-gray-100">{{ p.name }}</div>
      </div>
    </div>

    <a href="https://github.com/devlive-community/codeforge/releases" target="_blank" rel="noopener"
       class="inline-block mt-10 px-8 py-4 rounded-xl bg-green-500 hover:bg-green-600 text-white text-lg font-medium shadow-lg shadow-green-500/20 transition-colors">
      点我前往下载
    </a>
  </section>
</template>

<script setup lang="ts">
import {latestRelease} from '../content/releases'

const platforms = [
  {name: '适配 Windows', icon: '🪟'},
  {name: '适配 macOS', icon: '🍎'}
]
</script>
