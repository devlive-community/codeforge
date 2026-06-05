<template>
  <span class="flex items-center text-xs text-gray-500 min-w-0">
    <template v-for="(seg, i) in segments" :key="i">
      <ChevronRight v-if="i > 0" class="w-3 h-3 mx-0.5 text-gray-300 dark:text-gray-600 flex-shrink-0"/>
      <button class="hover:text-blue-500 truncate max-w-[160px] cursor-pointer"
              :title="`在访达中显示：${seg.full}`"
              @click="emit('reveal', seg.full)">
        {{ seg.name }}
      </button>
    </template>
    <span v-if="dirty" class="ml-1 text-amber-500 flex-shrink-0" title="有未保存的修改">●</span>
  </span>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {ChevronRight} from 'lucide-vue-next'

const props = defineProps<{
  path: string
  rootDir?: string | null
  dirty?: boolean
}>()
const emit = defineEmits<{ reveal: [path: string] }>()

interface Seg { name: string; full: string }

const segments = computed<Seg[]>(() => {
  const p = props.path
  const root = props.rootDir
  // 在已打开文件夹内：以文件夹为根显示相对层级；否则只显示文件名
  if (root && p.startsWith(root)) {
    const rel = p.slice(root.length).replace(/^[\\/]/, '')
    const parts = rel.split(/[\\/]/).filter(Boolean)
    const segs: Seg[] = []
    let acc = root.replace(/[\\/]$/, '')
    for (const part of parts) {
      acc = `${acc}/${part}`
      segs.push({name: part, full: acc})
    }
    return segs
  }
  const name = p.split(/[\\/]/).pop() || p
  return [{name, full: p}]
})
</script>
