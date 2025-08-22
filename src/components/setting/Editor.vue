<template>
  <div v-if="editorConfig" class="space-y-2">
    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        是否使用 tab 缩进
      </label>
      <div class="flex gap-2">
        <Switch v-model="editorConfig.indent_with_tab"/>
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        缩进空格数
      </label>
      <div class="flex gap-2">
        <Number v-model="editorConfig.tab_size" :min="1" :max="8" placeholder="缩进空格数"/>
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        编辑器主题
      </label>
      <div class="flex gap-2">
        <Select v-model="editorConfig.theme" class="w-1/4" placeholder="选择编辑器主题" :options="themeOptions"/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useEditorConfig } from '../../composables/useEditorConfig'
import Select from '../../ui/Select.vue'
import Switch from '../../ui/Switch.vue'
import Number from '../../ui/Number.vue'

const emit = defineEmits<{
  'settings-changed': [config: any]
  'error': [message: string]
}>()

const {
  editorConfig,
  themeOptions,
  loadConfig
} = useEditorConfig(emit)

onMounted(async () => {
  await loadConfig()
})
</script>
