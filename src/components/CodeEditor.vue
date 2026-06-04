<template>
  <div class="flex bg-white h-full relative">
    <Codemirror v-if="isReady"
                :style="{ width: '100%', height: '100%', fontSize: editorConfig?.font_size ? `${editorConfig.font_size}px` : undefined }"
                :model-value="modelValue"
                :extensions="extensions"
                :indent-with-tab="editorConfig?.indent_with_tab"
                :tab-size="editorConfig?.tab_size"
                @change="handleInput"
                @ready="onReady"/>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { Codemirror } from 'vue-codemirror'
import { useCodeMirrorEditor } from '../composables/useCodeMirrorEditor'

const props = defineProps<{
  modelValue: string
  language?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'ready': [view: any]
}>()

const {
  isReady,
  extensions,
  editorConfig,
  initializeEditor
} = useCodeMirrorEditor(props)

const handleInput = (value: string) => {
  emit('update:modelValue', value)
}

const onReady = (payload: any) => {
  emit('ready', payload.view)
}

onMounted(async () => {
  await initializeEditor()
})
</script>
