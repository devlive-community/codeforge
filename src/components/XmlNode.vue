<template>
  <div>
    <div class="flex items-start hover:bg-gray-100 dark:hover:bg-gray-800/60 rounded px-1"
         :style="{ paddingLeft: `${depth * 14 + 4}px` }">
      <span v-if="hasElementChildren"
            class="cursor-pointer select-none w-4 flex-shrink-0 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
            @click="open = !open">{{ open ? '▾' : '▸' }}</span>
      <span v-else class="w-4 flex-shrink-0"></span>

      <span class="break-all">
        <span class="text-blue-600 dark:text-blue-400">&lt;{{ tag }}</span><!--
     --><span v-for="a in attrs" :key="a.name"> <span class="text-purple-600 dark:text-purple-400">{{ a.name }}</span><span class="text-gray-400">=</span><span class="text-green-600 dark:text-green-400">"{{ a.value }}"</span></span><!--
     --><template v-if="selfClosing"><span class="text-blue-600 dark:text-blue-400">/&gt;</span></template><!--
     --><template v-else><span class="text-blue-600 dark:text-blue-400">&gt;</span><!--
       --><template v-if="!hasElementChildren"><span class="text-gray-700 dark:text-gray-300">{{ text }}</span><span class="text-blue-600 dark:text-blue-400">&lt;/{{ tag }}&gt;</span></template><!--
       --><template v-else-if="!open"><span class="text-gray-400 cursor-pointer" @click="open = true">…</span><span class="text-blue-600 dark:text-blue-400">&lt;/{{ tag }}&gt;</span></template><!--
     --></template>
      </span>
    </div>

    <template v-if="hasElementChildren && open">
      <XmlNode v-for="(c, i) in elementChildren" :key="i" :node="c" :depth="depth + 1"/>
      <div class="text-blue-600 dark:text-blue-400" :style="{ paddingLeft: `${depth * 14 + 4 + 16}px` }">&lt;/{{ tag }}&gt;</div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'

const props = defineProps<{
  node: Element
  depth: number
}>()

const tag = computed(() => props.node.tagName)

const attrs = computed(() =>
    Array.from(props.node.attributes).map(a => ({name: a.name, value: a.value}))
)

const elementChildren = computed(() =>
    Array.from(props.node.children) as Element[]
)
const hasElementChildren = computed(() => elementChildren.value.length > 0)

// 无元素子节点时的文本内容
const text = computed(() => (props.node.textContent || '').trim())

// 无子元素且无文本 → 自闭合
const selfClosing = computed(() => !hasElementChildren.value && !text.value)

const open = ref(props.depth < 3)
</script>
