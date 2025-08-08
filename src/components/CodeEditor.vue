<template>
  <div class="flex bg-white h-full">
    <!-- 行号 -->
    <div ref="lineNumbersRef"
         class="bg-gray-50 text-gray-400 text-sm font-mono py-4 px-3 select-none border-r border-gray-200 overflow-hidden">
      <div v-for="(num, index) in lineNumbers" :key="index" class="h-6 leading-6 text-right">
        {{ num }}
      </div>
    </div>

    <!-- 代码输入框 -->
    <textarea ref="textareaRef"
              :value="modelValue"
              @input="handleInput"
              @keydown="handleKeyDown"
              @scroll="handleScroll"
              class="flex-1 p-4 font-mono text-sm leading-6 resize-none outline-none bg-white"
              placeholder="在此输入代码..."
              spellcheck="false">
    </textarea>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref } from 'vue'

const props = defineProps<{
  modelValue: string
  language?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const textareaRef = ref<HTMLTextAreaElement>()
const lineNumbersRef = ref<HTMLElement>()

const lineNumbers = computed(() => {
  const lines = props.modelValue.split('\n')
  return lines.map((_, index) => String(index + 1))
})

const handleInput = (e: Event) => {
  const target = e.target as HTMLTextAreaElement
  emit('update:modelValue', target.value)
}

const handleKeyDown = async (e: KeyboardEvent) => {
  if (e.key === 'Tab') {
    e.preventDefault()
    const target = e.target as HTMLTextAreaElement
    const start = target.selectionStart
    const end = target.selectionEnd
    const newValue = props.modelValue.substring(0, start) + '    ' + props.modelValue.substring(end)
    emit('update:modelValue', newValue)

    await nextTick()
    target.selectionStart = target.selectionEnd = start + 4
  }
}

const handleScroll = (e: Event) => {
  const target = e.target as HTMLTextAreaElement
  if (lineNumbersRef.value) {
    lineNumbersRef.value.scrollTop = target.scrollTop
  }
}
</script>