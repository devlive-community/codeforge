<template>
  <div v-if="editorConfig" class="space-y-2">
    <Label label="是否使用 Tab 缩进">
      <Switch v-model="editorConfig.indent_with_tab"/>
    </Label>

    <Label label="缩进空格数">
      <Number v-model="editorConfig.tab_size" :min="1" :max="8" placeholder="缩进空格数"/>
    </Label>

    <Label label="字体大小">
      <Number v-model="editorConfig.font_size" :min="1" :max="30" placeholder="字体大小"/>
    </Label>

    <Label label="编辑器主题">
      <Select v-model="editorConfig.theme" class="w-1/4" placeholder="选择编辑器主题" :options="themeOptions"/>
    </Label>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useEditorConfig } from '../../composables/useEditorConfig'
import Select from '../../ui/Select.vue'
import Switch from '../../ui/Switch.vue'
import Number from '../../ui/Number.vue'
import Label from '../../ui/Label.vue'

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
