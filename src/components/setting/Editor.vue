<template>
  <div v-if="editorConfig" class="space-y-2">
    <Label label="是否使用 Tab 缩进">
      <Switch v-model="editorConfig.indent_with_tab"/>
    </Label>

    <Label label="是否显示行号">
      <Switch v-model="editorConfig.show_line_numbers"/>
    </Label>

    <Label label="是否显示函数帮助信息">
      <Switch v-model="editorConfig.show_function_help"/>
    </Label>

    <Label label="是否显示空格省略">
      <Switch v-model="editorConfig.space_dot_omission"/>
    </Label>

    <Label label="缩进空格数">
      <Number v-model="editorConfig.tab_size" :min="1" :max="8" placeholder="缩进空格数"/>
    </Label>

    <Label label="编辑器字体">
      <div class="flex items-center space-x-2">
        <Input v-model="editorConfig.font_family" class="w-1/3" disabled placeholder="编辑器字体"/>
        <Button :icon="ALargeSmall" icon-only @click="selectFont"></Button>
      </div>
    </Label>

    <Label label="字体大小">
      <Number v-model="editorConfig.font_size" :min="1" :max="30" placeholder="字体大小"/>
    </Label>

    <Label label="打开文件大小上限 (MB)">
      <Number v-model="editorConfig.max_open_file_size" :min="1" :max="200" placeholder="超过该大小将拒绝打开"/>
    </Label>

    <Label label="运行未保存文件时">
      <Select v-model="editorConfig.run_save_strategy" class="w-1/3" placeholder="选择运行策略" :options="runSaveStrategyOptions"/>
    </Label>

    <Label label="编辑器主题">
      <Select v-model="editorConfig.theme" class="w-1/4" placeholder="选择编辑器主题" :options="themeOptions"/>
    </Label>
  </div>
</template>

<script setup lang="ts">
import {onMounted} from 'vue'
import {useEditorConfig} from '../../composables/useEditorConfig'
import Select from '../../ui/Select.vue'
import Switch from '../../ui/Switch.vue'
import Number from '../../ui/Number.vue'
import Label from '../../ui/Label.vue'
import Input from "../../ui/Input.vue";
import Button from "../../ui/Button.vue";
import {ALargeSmall} from "lucide-vue-next";

const emit = defineEmits<{
  'settings-changed': [config: any]
  'error': [message: string]
}>()

const runSaveStrategyOptions = [
  {label: '自动保存后运行', value: 'auto-save'},
  {label: '每次询问', value: 'ask'},
  {label: '运行副本(不保存)', value: 'temp-copy'}
]

const {
  editorConfig,
  themeOptions,
  loadConfig,
  selectFont
} = useEditorConfig(emit)

onMounted(async () => {
  await loadConfig()
})
</script>
