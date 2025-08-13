<template>
  <div class="flex bg-white h-full relative">
    <Codemirror v-if="isReady"
                style="width: 100%; height: 100%"
                :model-value="modelValue"
                :extensions="extensions"
                @change="handleInput"/>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref, watch } from 'vue'
import { Codemirror } from 'vue-codemirror'
import { python } from '@codemirror/lang-python'
import { javascript } from '@codemirror/lang-javascript'
import { go } from '@codemirror/lang-go'
import { githubLight } from '@uiw/codemirror-themes-all'

const props = defineProps<{
  modelValue: string
  language?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const handleInput = (value: string) => {
  emit('update:modelValue', value)
}

const isReady = ref(false)
const extensions = ref<[]>([])

// 获取语言扩展
const getLanguageExtension = (language: string): any | null => {
  switch (language) {
    case 'python2':
    case 'python3':
      return python()
    case 'nodejs':
      return javascript()
    case 'go':
      return go()
    default:
      return null
  }
}

// 更新扩展的函数
const updateExtensions = async () => {
  const result = [githubLight]

  if (props.language) {
    const langExtension = getLanguageExtension(props.language)
    if (langExtension) {
      result.push(langExtension)
    }
  }

  extensions.value = result as any

  // 如果组件还没准备好，等待下一个 tick 后设置为准备好
  if (!isReady.value) {
    await nextTick()
    isReady.value = true
  }
}

// 监听语言变化
watch(() => props.language, async () => {
  isReady.value = false
  await nextTick()
  await updateExtensions()
}, { immediate: false })

// 组件挂载时初始化
onMounted(async () => {
  await updateExtensions()
})
</script>
