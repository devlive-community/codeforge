<template>
  <div class="max-w-7xl mx-auto px-5 py-10">
    <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_250px] gap-10">
      <!-- 博客正文 -->
      <article class="prose-doc min-w-0">
        <slot/>
        
        <!-- 上一篇/下一篇导航 -->
        <nav v-if="prevBlog || nextBlog" class="not-prose mt-16 pt-8 border-t border-slate-200 dark:border-slate-800">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <!-- 上一篇 -->
            <RouterLink v-if="prevBlog" :to="`/blog/${prevBlog.slug}`"
                        class="group flex flex-col p-5 rounded-xl border-2 border-slate-200 dark:border-slate-800 hover:border-brand-400 dark:hover:border-brand-500 hover:shadow-lg transition-all">
              <span class="text-xs font-semibold text-slate-500 dark:text-slate-400 mb-2">← 上一篇</span>
              <span class="font-medium text-slate-900 dark:text-white group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors line-clamp-2">
                {{ prevBlog.title }}
              </span>
            </RouterLink>
            <div v-else></div>
            
            <!-- 下一篇 -->
            <RouterLink v-if="nextBlog" :to="`/blog/${nextBlog.slug}`"
                        class="group flex flex-col p-5 rounded-xl border-2 border-slate-200 dark:border-slate-800 hover:border-brand-400 dark:hover:border-brand-500 hover:shadow-lg transition-all text-right">
              <span class="text-xs font-semibold text-slate-500 dark:text-slate-400 mb-2">下一篇 →</span>
              <span class="font-medium text-slate-900 dark:text-white group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors line-clamp-2">
                {{ nextBlog.title }}
              </span>
            </RouterLink>
          </div>
        </nav>
        
        <!-- 返回列表 -->
        <div class="not-prose mt-8 text-center">
          <RouterLink to="/blog" 
                      class="inline-flex items-center gap-2 px-5 py-3 rounded-xl text-slate-600 dark:text-slate-400 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16l-4-4m0 0l4-4m-4 4h18"/>
            </svg>
            <span>返回博客列表</span>
          </RouterLink>
        </div>
      </article>
      
      <!-- 右侧目录 -->
      <aside class="hidden lg:block">
        <div class="sticky top-24">
          <p class="px-3 text-xs font-semibold uppercase tracking-wider text-slate-400 mb-3">目录</p>
          <nav class="space-y-1 max-h-[70vh] overflow-y-auto">
            <a v-for="heading in headings" :key="heading.id"
               :href="`#${heading.id}`"
               :class="[
                 'block px-3 py-1.5 rounded-lg text-sm transition-colors',
                 heading.level === 2 ? 'pl-3' : 'pl-6',
                 activeId === heading.id 
                   ? 'bg-brand-50 dark:bg-brand-900/30 text-brand-700 dark:text-brand-300 font-medium'
                   : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60'
               ]"
               @click.prevent="scrollToHeading(heading.id)">
              {{ heading.text }}
            </a>
          </nav>
        </div>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref} from 'vue'
import {useRoute} from 'vue-router'
import {blogs} from '../content/blogs'

const route = useRoute()

const currentSlug = computed(() => {
  const match = route.path.match(/\/blog\/([^/]+)/)
  return match ? match[1] : null
})

const currentIndex = computed(() => {
  if (!currentSlug.value) return -1
  return blogs.findIndex(b => b.slug === currentSlug.value)
})

const prevBlog = computed(() => {
  const idx = currentIndex.value
  return idx > 0 ? blogs[idx - 1] : null
})

const nextBlog = computed(() => {
  const idx = currentIndex.value
  return idx >= 0 && idx < blogs.length - 1 ? blogs[idx + 1] : null
})

// 目录相关
interface Heading {
  id: string
  text: string
  level: number
}

const headings = ref<Heading[]>([])
const activeId = ref<string>('')

const extractHeadings = () => {
  const article = document.querySelector('.prose-doc')
  if (!article) return
  
  const h2s = article.querySelectorAll('h2, h3')
  headings.value = Array.from(h2s).map(h => ({
    id: h.id,
    text: h.textContent || '',
    level: parseInt(h.tagName[1])
  }))
}

const scrollToHeading = (id: string) => {
  const element = document.getElementById(id)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }
}

const updateActiveHeading = () => {
  const headingElements = headings.value.map(h => document.getElementById(h.id)).filter(Boolean)
  
  // 找到当前在视口顶部的标题
  for (let i = headingElements.length - 1; i >= 0; i--) {
    const element = headingElements[i]
    if (element && element.getBoundingClientRect().top <= 100) {
      activeId.value = element.id
      return
    }
  }
  
  // 如果没有标题在视口顶部，默认激活第一个
  if (headingElements.length > 0) {
    activeId.value = headingElements[0]!.id
  }
}

onMounted(() => {
  // 等待 markdown 渲染完成后提取标题
  setTimeout(() => {
    extractHeadings()
    updateActiveHeading()
  }, 100)
  
  window.addEventListener('scroll', updateActiveHeading)
})

onUnmounted(() => {
  window.removeEventListener('scroll', updateActiveHeading)
})
</script>
