<template>
  <div v-if="editorConfig" class="space-y-2">
    <Label :label="t('settings.editor.indentWithTab')">
      <Switch v-model="editorConfig.indent_with_tab"/>
    </Label>

    <Label :label="t('settings.editor.showLineNumbers')">
      <Switch v-model="editorConfig.show_line_numbers"/>
    </Label>

    <Label :label="t('settings.editor.showFunctionHelp')">
      <Switch v-model="editorConfig.show_function_help"/>
    </Label>

    <Label :label="t('settings.editor.spaceDotOmission')">
      <Switch v-model="editorConfig.space_dot_omission"/>
    </Label>

    <Label :label="t('settings.editor.showMinimap')">
      <Switch v-model="editorConfig.show_minimap"/>
    </Label>

    <Label :label="t('settings.editor.showStickyScroll')">
      <Switch v-model="editorConfig.show_sticky_scroll"/>
    </Label>

    <Label :label="t('settings.editor.wordWrap')">
      <Switch v-model="editorConfig.word_wrap"/>
    </Label>

    <Label :label="t('settings.editor.showIndentGuides')">
      <Switch v-model="editorConfig.show_indent_guides"/>
    </Label>

    <Label :label="t('settings.editor.renderWhitespace')">
      <Switch v-model="editorConfig.render_whitespace"/>
    </Label>

    <Label :label="t('settings.editor.tabSize')">
      <Number v-model="editorConfig.tab_size" :min="1" :max="8" :placeholder="t('settings.editor.tabSize')"/>
    </Label>

    <Label :label="t('settings.editor.font')">
      <div class="flex items-center space-x-2">
        <Input v-model="editorConfig.font_family" class="w-1/3" disabled :placeholder="t('settings.editor.font')"/>
        <Button :icon="ALargeSmall" icon-only @click="selectFont"></Button>
      </div>
    </Label>

    <Label :label="t('settings.editor.fontSize')">
      <Number v-model="editorConfig.font_size" :min="1" :max="30" :placeholder="t('settings.editor.fontSize')"/>
    </Label>

    <Label :label="t('settings.editor.theme')">
      <Select v-model="editorConfig.theme" class="w-1/4" :placeholder="t('settings.editor.selectTheme')" :options="themeOptions"/>
    </Label>
  </div>
</template>

<!-- 注：「打开文件大小上限」「运行未保存文件时」属通用行为，已移至通用设置 -->

<script setup lang="ts">
import {onMounted} from 'vue'
import {useI18n} from 'vue-i18n'
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

const {t} = useI18n()

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
