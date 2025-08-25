<template>
  <div class="-mt-2">
    <Tabs v-model="activePlugin"
          type="card"
          size="md"
          position="left"
          :tab-button-class="['w-48']"
          :nav-class="['max-h-[70vh] overflow-y-auto']"
          :tabs="tabsPluginData"
          @change="handleTabChange">
      <template #[activePlugin]="{ tab }">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center space-x-2">
          <img :src="`/icons/${activePlugin.replace(/\d+$/, '')}.svg`" class="w-6 h-6" :alt="tab.label"/>
          <span>{{ `语言 [ ${tab.label} ] 配置` }}</span>
        </h3>

        <Tabs v-model="activeTab"
              type="card"
              size="md"
              :tabs="tabsData"
              :nav-class="['w-full', 'justify-center']">
          <template #general>
            <div class="space-y-4">
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
            <Label label="语言环境目录">
              <div class="flex gap-2">
                <Input v-model="pluginConfig.execute_home" class="w-full" placeholder="选择语言环境目录路径"/>

                <Button type="primary"
                        :icon-only="true"
                        :icon="Folder"
                        @click="selectExecuteHome">
                </Button>
              </div>
            </Label>
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
            <Label label="超时时间(秒)">
              <Number v-model="pluginConfig.timeout" class="w-1/4" placeholder="超时时间(秒)，默认 30 秒"/>
            </Label>
          </template>
        </Tabs>
      </template>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import {onMounted} from 'vue'
import {Folder} from 'lucide-vue-next'
import {Codemirror} from 'vue-codemirror'
import Button from '../../ui/Button.vue'
import Tabs from '../../ui/Tabs.vue'
import Number from '../../ui/Number.vue'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
import {useLanguageSettings} from '../../composables/useLanguageSettings'
import type PluginConfig from '../../types/plugin'

const emit = defineEmits<{
  'settings-changed': [config: PluginConfig]
  'error': [message: string]
}>()

const {
  // 标签页状态
  activeTab,
  tabsData,

  // 插件配置
  activePlugin,
  tabsPluginData,
  pluginConfig,
  handleTabChange,
  selectExecuteHome,

  // 编辑器状态
  isEditorReady,
  currentExtensions,

  // 方法
  initialize
} = useLanguageSettings(emit)

onMounted(async () => {
  await initialize()
})
</script>
