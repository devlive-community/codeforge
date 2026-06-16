<template>
  <Modal v-model:show="isVisible" :title="t('settings.title')" size="5xl" :content-class="['min-h-[60vh]']" :close-on-backdrop="false" :close-on-esc="false" @close="closeSettings">
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

      <!-- 快捷键配置 -->
      <template #shortcut>
        <Shortcut v-if="activeTab === 'shortcut'"/>
      </template>

      <!-- AI 配置 -->
      <template #ai>
        <Ai v-if="activeTab === 'ai'"/>
      </template>

      <!-- 数据库连接 -->
      <template #database>
        <Database v-if="activeTab === 'database'"/>
      </template>

      <!-- 语言配置 -->
      <template #language>
        <Language v-if="activeTab === 'language'" @settings-changed="handleLanguageSettingsChanged"/>
      </template>

      <!-- 语言服务（LSP） -->
      <template #lsp>
        <Lsp v-if="activeTab === 'lsp'"/>
      </template>

      <!-- 网络配置 -->
      <template #network>
        <Network v-if="activeTab === 'network'" @settings-changed="handleNetworkSettingsChanged" @error="handleEditorError"/>
      </template>

      <!-- 缓存管理 -->
      <template #cache>
        <Cache v-if="activeTab === 'cache'"/>
      </template>

      <!-- 日志管理 -->
      <template #logs>
        <Logs v-if="activeTab === 'logs'" @settings-changed="handleNetworkSettingsChanged" @error="handleEditorError"/>
      </template>

      <!-- 语言包 -->
      <template #i18n>
        <I18n v-if="activeTab === 'i18n'"/>
      </template>
    </Tabs>
  </Modal>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import Modal from '../ui/Modal.vue'
import Tabs from '../ui/Tabs.vue'
import General from './setting/General.vue'
import Language from './setting/Language.vue'
import Editor from './setting/Editor.vue'
import Shortcut from './setting/Shortcut.vue'
import Ai from './setting/Ai.vue'
import Database from './setting/Database.vue'
import Lsp from './setting/Lsp.vue'
import Network from './setting/Network.vue'
import Cache from './setting/Cache.vue'
import Logs from './setting/Logs.vue'
import I18n from './setting/I18n.vue'
import { useSettings } from '../composables/useSettings.ts'

const emit = defineEmits<{
  close: []
  'settings-changed': [config: any]
}>()

const {t} = useI18n()

const {
  isVisible,
  activeTab,
  tabsData,
  handleEditorSettingsChanged,
  handleLanguageSettingsChanged,
  handleNetworkSettingsChanged,
  handleEditorError,
  closeSettings,
  initialize
} = useSettings(emit)

onMounted(async () => {
  await initialize()
})
</script>
