<template>
  <div class="max-w-6xl mx-auto px-5 py-16">
    <h1 class="text-5xl font-extrabold tracking-tight text-slate-900 dark:text-white mb-4">
      技术博客
    </h1>
    <p class="text-lg text-slate-600 dark:text-slate-400 mb-12">
      分享开发经验、使用教程和技术洞察
    </p>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <RouterLink v-for="blog in blogs" :key="blog.slug" :to="`/blog/${blog.slug}`"
                  class="group relative flex flex-col p-6 rounded-2xl border-2 border-slate-200 dark:border-slate-800 bg-gradient-to-br from-white to-slate-50 dark:from-slate-900 dark:to-slate-900/50 hover:border-brand-400 dark:hover:border-brand-500 hover:shadow-xl hover:shadow-brand-500/10 dark:hover:shadow-brand-500/20 transition-all duration-300 hover:-translate-y-1">
        <!-- 标题 -->
        <h2 class="text-xl font-bold text-slate-900 dark:text-white group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors mb-3">
          {{ blog.title }}
        </h2>
        
        <!-- 元信息 -->
        <div class="flex items-center gap-4 text-sm text-slate-500 dark:text-slate-400 mb-4">
          <span v-if="blog.date" class="flex items-center gap-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
            </svg>
            {{ blog.date }}
          </span>
          <span v-if="blog.author" class="flex items-center gap-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
            </svg>
            {{ blog.author }}
          </span>
        </div>
        
        <!-- 描述 -->
        <p v-if="blog.description" class="text-sm text-slate-600 dark:text-slate-300 leading-relaxed line-clamp-2 mb-4 flex-1">
          {{ blog.description }}
        </p>
        
        <!-- 标签 -->
        <div v-if="blog.tags && blog.tags.length > 0" class="flex flex-wrap gap-2">
          <span v-for="tag in blog.tags" :key="tag" 
                class="px-2.5 py-1 rounded-lg bg-slate-100 dark:bg-slate-800 text-xs font-medium text-slate-600 dark:text-slate-300">
            {{ tag }}
          </span>
        </div>
        
        <!-- 装饰渐变 -->
        <div class="absolute inset-0 rounded-2xl bg-gradient-to-br from-brand-500/0 via-brand-500/0 to-brand-500/10 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import {blogs} from '../content/blogs'
</script>
