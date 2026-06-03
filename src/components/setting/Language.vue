<template>
  <div class="-mt-2">
    <Tabs v-model="activePlugin"
          type="card"
          size="md"
          position="left"
          :tab-button-class="['!p-1 ']"
          :header-class="['w-64', 'flex-shrink-0']"
          :nav-class="['max-h-[65vh]', 'overflow-y-auto', 'w-full']"
          :tabs="filteredPluginData"
          @change="handleTabChange">
      <template #nav-header>
        <div class="flex items-center space-x-2">
          <Input v-model="languageFilter"
                 :prefix-icon="Search"
                 clearable
                 size="sm"
                 class="flex-1"
                 placeholder="筛选语言"/>
          <Button @click="showAddCustomLanguage = true"
                  size="sm"
                  :icon="Plus"
                  :icon-only="true"
                  title="添加自定义语言"/>
        </div>

        <!-- 无搜索结果提示 -->
        <div v-if="languageFilter.trim() && filteredPluginData.length === 0"
             class="mt-3 px-3 py-4 text-center rounded-lg bg-gray-50 dark:bg-gray-800">
          <Search class="w-6 h-6 mx-auto text-gray-400"/>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-300">未找到匹配的语言</p>
          <p class="mt-1 text-xs text-gray-400">
            没有你需要的语言？
            <button class="text-blue-500 hover:text-blue-600 hover:underline cursor-pointer" @click="openIssues">
              提交 Issue 反馈
            </button>
          </p>
        </div>
      </template>
      <template #tab-button="{ tab }">
        <div class="flex items-center w-full px-3 py-2 space-x-2">
          <Switch v-model="pluginEnabledStates[tab.key as string]"
                  size="sm"
                  @click.stop
                  @change="(value, event) => handlePluginToggle(tab.key as string, value, event)"/>
          <div class="flex items-center space-x-2 flex-1">
            <img v-if="tab.svgUrl" :src="tab.svgUrl" class="w-5 h-5" :alt="tab.label"/>
            <span>{{ tab.label }}</span>
          </div>
          <button v-if="isCustomLanguage(tab.key as string)"
                  @click.stop="confirmDelete(tab.key as string)"
                  class="text-red-500 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300 p-1 hover:cursor-pointer">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
      </template>
      <template #[activePlugin]="{ tab }">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center space-x-2">
          <img v-if="tab.svgUrl" :src="tab.svgUrl" class="w-6 h-6" :alt="tab.label"/>
          <span>{{ `语言 [ ${tab.label} ] 配置` }}</span>
        </h3>

        <Tabs v-model="activeTab"
              type="card"
              size="md"
              :tabs="tabsData"
              :nav-class="['w-full', 'justify-center']">
          <template #general>
            <div class="space-y-4">
              <Label label="启用插件">
                <Switch v-model="pluginConfig.enabled"/>
              </Label>

              <Label label="编译前执行的命令">
                <Input v-model="pluginConfig.before_compile" class="w-full" placeholder="编译前执行的命令"/>
              </Label>

              <Label label="执行的命令">
                <Input v-model="pluginConfig.run_command" class="w-full" placeholder="执行的命令"/>
                <div class="flex flex-col mx-2 mt-2 space-y-1.5">
                  <div class="flex text-sm font-medium text-gray-700 dark:text-gray-300 ml-1 space-x-4">
                    <div class="font-bold">$filename</div>
                    <div>执行的源文件或临时生成的文件</div>
                  </div>
                  <div class="flex text-sm font-medium text-gray-700 dark:text-gray-300 ml-1 space-x-4">
                    <div class="font-bold">$classname</div>
                    <div>执行的类名,如果没有则为空</div>
                  </div>
                </div>
              </Label>

              <Label label="编译完成后执行的命令">
                <Input v-model="pluginConfig.after_compile" class="w-full" placeholder="编译完成后执行的命令"/>
              </Label>

              <Label label="文件后缀名">
                <Input v-model="pluginConfig.extension" class="w-full" placeholder="输入文件后缀名"/>
              </Label>
            </div>
          </template>

          <template #environment>
            <EnvironmentManager :language="activePlugin"
                                :execute-home="pluginConfig.execute_home as any"
                                @update:execute-home="(value) => pluginConfig.execute_home = value"
                                @select-directory="selectExecuteHome"/>
          </template>

          <template #template>
            <div class="w-[98%]">
              <Codemirror v-if="isEditorReady && pluginConfig.template !== undefined"
                          style="width: 102%; height: 380px"
                          v-model="pluginConfig.template"
                          :extensions="currentExtensions"
                          class="flex-1 border border-gray-300 dark:border-gray-600 rounded-md overflow-hidden"/>
              <div v-else class="flex-1 flex items-center justify-center h-64 border border-gray-300 dark:border-gray-600 rounded-md">
                <div class="text-gray-500">加载编辑器中...</div>
              </div>
            </div>
          </template>

          <template #advanced>
            <div class="space-y-4">
              <Label label="超时时间(秒)">
                <Number v-model="pluginConfig.timeout" class="w-1/4" placeholder="超时时间(秒)，默认 30 秒"/>
              </Label>

              <Label label="输出类型">
                <Select v-model="pluginConfig.console_type" placeholder="请选择输出类型" class="w-1/3" :options="consoleTypes"></Select>
              </Label>
            </div>
          </template>
        </Tabs>
      </template>
    </Tabs>

    <Modal v-model:show="showAddCustomLanguage" :close-on-backdrop="false" :close-on-esc="false" title="添加自定义语言">
      <div class="space-y-4">
        <Label label="语言标识">
          <Input v-model="newLanguage.language" class="w-full" placeholder="例如: dart, perl"/>
        </Label>
        <Label label="语言名称">
          <Input v-model="newLanguageName" class="w-full" placeholder="例如: Dart, Perl"/>
        </Label>
        <Label label="文件扩展名">
          <Input v-model="newLanguage.extension" class="w-full" placeholder="例如: dart, pl, sh"/>
        </Label>
        <Label label="语言图标">
          <div class="flex items-center space-x-2">
            <Button @click="selectIconFile" variant="outline" size="sm">选择图标文件</Button>
            <span v-if="selectedIconFile" class="text-sm text-gray-600 dark:text-gray-400">{{ selectedIconFile.name }}</span>
          </div>
        </Label>
        <div class="flex justify-end space-x-2">
          <Button @click="showAddCustomLanguage = false" type="secondary" size="sm">取消</Button>
          <Button @click="addCustomLanguage" size="sm">添加</Button>
        </div>
      </div>
    </Modal>

    <Modal v-model:show="showDeleteConfirm" title="确认删除" size="sm">
      <div class="space-y-4">
        <p class="text-gray-700 dark:text-gray-300">
          确定要删除自定义语言 <strong>{{ languageToDelete }}</strong> 吗？
        </p>
        <div class="flex justify-end space-x-2">
          <Button @click="showDeleteConfirm = false" type="secondary" size="sm">取消</Button>
          <Button @click="deleteCustomLanguage" size="sm" class="bg-red-500 hover:bg-red-600 text-white">删除</Button>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { Plus, Search } from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { open as openUrl } from '@tauri-apps/plugin-shell'
import { readFile } from '@tauri-apps/plugin-fs'
import { Codemirror } from 'vue-codemirror'
import Tabs from '../../ui/Tabs.vue'
import Number from '../../ui/Number.vue'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
import Button from '../../ui/Button.vue'
import Modal from '../../ui/Modal.vue'
import { useLanguageSettings } from '../../composables/useLanguageSettings'
import type PluginConfig from '../../types/plugin'
import Select from "../../ui/Select.vue";
import Switch from '../../ui/Switch.vue'
import EnvironmentManager from './EnvironmentManager.vue'
import { useToast } from '../../plugins/toast'

const emit = defineEmits<{
  'settings-changed': [config: PluginConfig]
  'error': [message: string]
}>()

const toast = useToast()
const languageFilter = ref('')
const showAddCustomLanguage = ref(false)
const showDeleteConfirm = ref(false)
const languageToDelete = ref('')
const newLanguageName = ref('')
const newLanguageIcon = ref('')
const selectedIconFile = ref<File | null>(null)
const customLanguages = ref<string[]>([])
const newLanguage = ref<PluginConfig>({
  enabled: true,
  execute_home: undefined,
  extension: '',
  language: '',
  before_compile: undefined,
  after_compile: undefined,
  run_command: undefined,
  template: undefined,
  timeout: 30,
  console_type: 'console'
})

const {
  activeTab,
  tabsData,
  consoleTypes,
  activePlugin,
  tabsPluginData,
  pluginConfig,
  pluginEnabledStates,
  handleTabChange,
  handlePluginToggle,
  selectExecuteHome,
  isEditorReady,
  currentExtensions,
  initialize,
  reloadLanguages
} = useLanguageSettings(emit)

// 根据筛选关键字过滤左侧语言列表（按名称或标识匹配）
const filteredPluginData = computed(() => {
  const keyword = languageFilter.value.trim().toLowerCase()
  if (!keyword) {
    return tabsPluginData.value
  }
  return tabsPluginData.value.filter((tab: any) => {
    const label = String(tab.label || '').toLowerCase()
    const key = String(tab.key || '').toLowerCase()
    return label.includes(keyword) || key.includes(keyword)
  })
})

const openIssues = async () => {
  try {
    await openUrl('https://github.com/devlive-community/codeforge/issues')
  }
  catch (error) {
    toast.error('打开链接失败: ' + error)
  }
}

const selectIconFile = async () => {
  try {
    const selected = await openDialog({
      multiple: false,
      filters: [{
        name: '图片文件',
        extensions: ['svg', 'png', 'jpg', 'jpeg', 'gif', 'webp']
      }]
    })

    if (selected) {
      const filePath = selected as string
      const fileName = filePath.split('/').pop() || ''
      selectedIconFile.value = { name: fileName } as File
      newLanguageIcon.value = filePath
    }
  }
  catch (error) {
    toast.error('选择文件失败: ' + error)
  }
}

const isCustomLanguage = (language: string) => {
  return customLanguages.value.includes(language)
}

const confirmDelete = (language: string) => {
  languageToDelete.value = language
  showDeleteConfirm.value = true
}

const deleteCustomLanguage = async () => {
  const language = languageToDelete.value
  showDeleteConfirm.value = false

  try {
    await invoke('remove_custom_plugin', { language })
    toast.success('自定义语言已删除')
    await reloadLanguages()
    await loadCustomLanguages()
  }
  catch (error: any) {
    toast.error('删除失败: ' + error)
  }
}

const loadCustomLanguages = async () => {
  try {
    const plugins = await invoke<PluginConfig[]>('get_custom_plugins')
    customLanguages.value = plugins.map(p => p.language)
  }
  catch (error) {
    console.error('加载自定义语言列表失败:', error)
  }
}

const addCustomLanguage = async () => {
  if (!newLanguage.value.language || !newLanguageName.value) {
    toast.error('请填写语言标识和语言名称')
    return
  }

  if (!newLanguage.value.extension) {
    toast.error('请填写文件扩展名')
    return
  }

  try {
    if (selectedIconFile.value && newLanguageIcon.value) {
      const fileData = await readFile(newLanguageIcon.value)
      const iconData = Array.from(fileData)
      const fileExtension = newLanguageIcon.value.split('.').pop() || 'svg'

      const iconPath = await invoke<string>('save_custom_icon', {
        language: newLanguage.value.language,
        iconData: iconData,
        fileExtension: fileExtension
      })

      newLanguage.value.icon_path = iconPath
    }

    await invoke('add_custom_plugin', { config: newLanguage.value })
    toast.success('自定义语言添加成功')
    showAddCustomLanguage.value = false

    newLanguage.value = {
      enabled: true,
      execute_home: undefined,
      extension: '',
      language: '',
      before_compile: undefined,
      after_compile: undefined,
      run_command: undefined,
      template: undefined,
      timeout: 30,
      console_type: 'console'
    }
    newLanguageName.value = ''
    newLanguageIcon.value = ''
    selectedIconFile.value = null

    await reloadLanguages()
    await loadCustomLanguages()
  }
  catch (error: any) {
    toast.error('添加失败: ' + error)
  }
}

onMounted(async () => {
  await initialize()
  await loadCustomLanguages()
})
</script>
