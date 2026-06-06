<template>
  <div class="fixed top-0 right-0 bottom-0 z-40 w-[45%] min-w-[360px] max-w-[720px] bg-white dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700 shadow-2xl flex flex-col">
    <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
        <Eye class="w-4 h-4 text-gray-400"/>
        <span>实时预览</span>
        <span class="text-xs text-gray-400">· {{ kindLabel }}</span>
      </div>
      <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" title="关闭预览" @click="emit('close')">
        <X class="w-4 h-4"/>
      </button>
    </div>

    <div class="flex-1 overflow-auto">
      <!-- HTML：沙箱 iframe -->
      <iframe v-if="kind === 'html'"
              :srcdoc="content"
              class="w-full h-full border-0 bg-white"
              sandbox="allow-same-origin"/>
      <!-- Markdown：渲染后的 HTML -->
      <div v-else-if="kind === 'markdown'"
           class="markdown-body px-6 py-5 text-sm text-gray-800 dark:text-gray-200"
           v-html="rendered"/>
      <!-- 不支持的类型 -->
      <div v-else class="px-6 py-10 text-center text-sm text-gray-400">
        当前文件类型不支持预览（仅支持 Markdown 与 HTML）
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {Eye, X} from 'lucide-vue-next'
import MarkdownIt from 'markdown-it'
import DOMPurify from 'dompurify'

const props = defineProps<{
  content: string
  language?: string
  fileName?: string | null
}>()
const emit = defineEmits<{ close: [] }>()

// 允许 Markdown 内的 HTML，渲染后用 DOMPurify 净化防 XSS
const md = new MarkdownIt({html: true, linkify: true, breaks: true})

// 根据语言或扩展名判断预览类型
const kind = computed<'markdown' | 'html' | 'none'>(() => {
  const lang = (props.language || '').toLowerCase()
  const name = (props.fileName || '').toLowerCase()
  if (lang === 'html' || name.endsWith('.html') || name.endsWith('.htm')) {
    return 'html'
  }
  if (lang === 'markdown' || name.endsWith('.md') || name.endsWith('.markdown')) {
    return 'markdown'
  }
  return 'none'
})

const kindLabel = computed(() => ({markdown: 'Markdown', html: 'HTML', none: '不支持'}[kind.value]))

const rendered = computed(() => (kind.value === 'markdown' ? DOMPurify.sanitize(md.render(props.content || '')) : ''))
</script>

<style scoped>
.markdown-body :deep(h1) { font-size: 1.6em; font-weight: 700; margin: 0.6em 0 0.4em; }
.markdown-body :deep(h2) { font-size: 1.35em; font-weight: 700; margin: 0.6em 0 0.4em; border-bottom: 1px solid rgba(128, 128, 128, 0.25); padding-bottom: 0.2em; }
.markdown-body :deep(h3) { font-size: 1.15em; font-weight: 600; margin: 0.6em 0 0.3em; }
.markdown-body :deep(p) { margin: 0.5em 0; line-height: 1.7; }
.markdown-body :deep(ul), .markdown-body :deep(ol) { margin: 0.5em 0; padding-left: 1.5em; }
.markdown-body :deep(li) { margin: 0.2em 0; }
.markdown-body :deep(code) { background: rgba(128, 128, 128, 0.18); padding: 0.1em 0.35em; border-radius: 4px; font-family: monospace; font-size: 0.9em; }
.markdown-body :deep(pre) { background: rgba(128, 128, 128, 0.15); padding: 0.8em 1em; border-radius: 6px; overflow-x: auto; margin: 0.6em 0; }
.markdown-body :deep(pre code) { background: none; padding: 0; }
.markdown-body :deep(blockquote) { border-left: 3px solid rgba(128, 128, 128, 0.4); padding-left: 1em; color: rgba(128, 128, 128, 0.95); margin: 0.6em 0; }
.markdown-body :deep(a) { color: #3b82f6; text-decoration: underline; }
.markdown-body :deep(table) { border-collapse: collapse; margin: 0.6em 0; }
.markdown-body :deep(th), .markdown-body :deep(td) { border: 1px solid rgba(128, 128, 128, 0.3); padding: 0.4em 0.7em; }
.markdown-body :deep(img) { max-width: 100%; }
.markdown-body :deep(hr) { border: none; border-top: 1px solid rgba(128, 128, 128, 0.3); margin: 1em 0; }
</style>
