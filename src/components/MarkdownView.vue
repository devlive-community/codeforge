<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <FileText class="w-3.5 h-3.5"/>
        <span>Markdown 预览</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ executionTime }} ms</span>
      </div>
      <button class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('clear')">清空</button>
    </div>

    <!-- 渲染内容 -->
    <div class="flex-1 overflow-auto">
      <div v-if="!output.trim()" class="text-gray-400 px-4 py-6 text-center text-sm">运行后在此查看 Markdown 预览</div>
      <div v-else class="markdown-body px-6 py-5 text-sm text-gray-800 dark:text-gray-200" v-html="rendered"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {FileText} from 'lucide-vue-next'
import MarkdownIt from 'markdown-it'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

const md = new MarkdownIt({html: false, linkify: true, breaks: true})
const rendered = computed(() => md.render(props.output || ''))
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
