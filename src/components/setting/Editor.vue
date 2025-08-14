<template>
  <div v-if="editorConfig" class="space-y-2">
    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        是否使用 tab 缩进
      </label>
      <div class="flex gap-2">
        <input v-model="editorConfig.indent_with_tab"
               type="checkbox"
               placeholder="超时时间(秒)，默认 30 秒"
               class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        缩进空格数
      </label>
      <div class="flex gap-2">
        <input v-model="editorConfig.tab_size"
               type="number"
               :disabled="!editorConfig.indent_with_tab"
               placeholder="缩进空格数，默认 2 秒"
               class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"
               :class="[!editorConfig.indent_with_tab ? 'opacity-50 cursor-not-allowed' : '']"/>
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        编辑器主题
      </label>
      <div class="flex gap-2">
        <Select v-model="editorConfig.theme"
                class="w-1/4"
                placeholder="选择编辑器主题"
                :options="themeOptions"/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Select from '../../ui/Select.vue'
import { useEditorConfig } from '../../composables/useEditorConfig'

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
