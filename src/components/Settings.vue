<template>
  <Modal v-model:show="isVisible" title="设置" size="5xl" :content-class="['min-h-[60vh]']" :close-on-backdrop="false" :close-on-esc="false" @close="closeSettings">
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

      <!-- 网络配置 -->
      <template #network>
        <Network v-if="activeTab === 'network'" @settings-changed="handleNetworkSettingsChanged" @error="handleEditorError"/>
      </template>

      <!-- 缓存管理 -->
      <template #cache>
        <Cache v-if="activeTab === 'cache'"/>
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
import Network from './setting/Network.vue'
import Cache from './setting/Cache.vue'
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
  handleNetworkSettingsChanged,
  handleEditorError,
  closeSettings,
  initialize
} = useSettings(emit)

onMounted(async () => {
  await initialize()
})
</script>
