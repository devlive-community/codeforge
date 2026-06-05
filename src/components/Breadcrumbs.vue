<template>
  <span class="flex items-center text-xs text-gray-500 w-max whitespace-nowrap">
    <template v-for="(seg, i) in segments" :key="i">
      <ChevronRight v-if="i > 0" class="w-3 h-3 mx-0.5 text-gray-300 dark:text-gray-600 flex-shrink-0"/>
      <button class="hover:text-blue-500 cursor-pointer flex-shrink-0"
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
  // 在已打开文件夹内：以文件夹名为首段，再展开相对层级
  if (root && p.startsWith(root)) {
    const base = root.replace(/[\\/]$/, '')
    const rootName = base.split(/[\\/]/).pop() || base
    const rel = p.slice(root.length).replace(/^[\\/]/, '')
    const parts = rel.split(/[\\/]/).filter(Boolean)
    const segs: Seg[] = [{name: rootName, full: base}]
    let acc = base
    for (const part of parts) {
      acc = `${acc}/${part}`
      segs.push({name: part, full: acc})
    }
    return segs
  }
  // 未打开文件夹：显示绝对路径（过长则保留末 4 段，full 仍为完整路径）
  const parts = p.split(/[\\/]/).filter(Boolean)
  const segs: Seg[] = []
  let acc = ''
  for (const part of parts) {
    acc = `${acc}/${part}`
    segs.push({name: part, full: acc})
  }
  return segs.length > 5 ? segs.slice(-4) : segs
})
</script>
