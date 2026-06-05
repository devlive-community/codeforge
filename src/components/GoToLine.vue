<template>
  <div class="fixed inset-0 z-50 flex justify-center pt-20" @click="emit('close')">
    <div class="w-[360px] max-w-[90vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700">
        <CornerDownRight class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="value"
               type="text"
               inputmode="numeric"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               :placeholder="`跳转到行（1 - ${maxLine}）`"
               @keydown.enter.prevent="submit"
               @keydown.esc.prevent="emit('close')"/>
      </div>
      <div class="px-3 py-1.5 text-xs text-gray-400">
        共 {{ maxLine }} 行 · 回车跳转
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {CornerDownRight} from 'lucide-vue-next'

const props = defineProps<{ maxLine: number }>()
const emit = defineEmits<{ go: [line: number]; close: [] }>()

const value = ref('')
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(() => inputRef.value?.focus())

const submit = () => {
  const n = parseInt(value.value.trim(), 10)
  if (!Number.isNaN(n) && n >= 1) {
    emit('go', Math.min(n, props.maxLine))
    emit('close')
  }
}
</script>
