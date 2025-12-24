<template>
  <Modal v-model:show="isVisible" title="设置" size="4xl" :content-class="['min-h-[60vh]']" :close-on-backdrop="false" :close-on-esc="false" @close="closeSettings">
    <Tabs v-model="activeTab"
          type="card"
          size="md"
          :nav-class="['w-full']"
          :tab-button-class="['flex', 'flex-col']"
          :tabs="tabsData">
      <!-- 通用配置 -->
      <template #general>
        <General/>
      </template>

      <!-- 编辑器配置 -->
      <template #editor>
        <Editor v-if="activeTab === 'editor'" @settings-changed="handleEditorSettingsChanged" @error="handleEditorError"/>
      </template>

      <!-- 语言配置 -->
      <template #language>
        <Language v-if="activeTab === 'language'" @settings-changed="handleLanguageSettingsChanged"/>
      </template>
    </Tabs>
  </Modal>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Modal from '../ui/Modal.vue'
import Tabs from '../ui/Tabs.vue'
import General from './setting/General.vue'
import Language from './setting/Language.vue'
import Editor from './setting/Editor.vue'
import { useSettings } from '../composables/useSettings.ts'

const emit = defineEmits<{
  close: []
  'settings-changed': [config: any]
}>()

const {
  isVisible,
  activeTab,
  tabsData,
  handleEditorSettingsChanged,
  handleLanguageSettingsChanged,
  handleEditorError,
  closeSettings,
  initialize
} = useSettings(emit)

onMounted(async () => {
  await initialize()
})
</script>
