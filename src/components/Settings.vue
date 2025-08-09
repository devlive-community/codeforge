<template>
  <div class="fixed inset-0 backdrop-blur-sm bg-white/20 dark:bg-gray-900/20 flex items-center justify-center z-50 transition-all duration-300 ease-out"
       :class="{ 'opacity-0': !isVisible, 'opacity-100': isVisible }">
    <div
        class="bg-white/95 dark:bg-gray-800/95 backdrop-blur-md rounded-xl shadow-2xl w-4xl p-6 transform transition-all duration-300 ease-out border border-white/20 dark:border-gray-700/30 max-h-[80vh] overflow-y-auto"
        :class="{
          'scale-95 opacity-0 translate-y-4': !isVisible,
          'scale-100 opacity-100 translate-y-0': isVisible
        }">

      <!-- 头部 -->
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">设置</h2>
        <button
            @click="closeSettings"
            class="text-gray-400 hover:cursor-pointer hover:text-gray-600 dark:hover:text-gray-200 transition-all duration-200 hover:scale-110 rounded-full p-1 hover:bg-gray-200 dark:hover:bg-gray-700">
          <X class="w-5 h-5"/>
        </button>
      </div>

      <!-- 日志设置 -->
      <div class="mb-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
          <FileText class="w-5 h-5 mr-2"/>
          日志设置
        </h3>

        <div class="space-y-4">
          <!-- 当前日志目录 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              当前日志目录
            </label>
            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-3 text-sm text-gray-600 dark:text-gray-400 font-mono">
              {{ currentLogDir || '加载中...' }}
            </div>
          </div>

          <!-- 修改日志目录 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              选择新的日志目录
            </label>
            <div class="flex gap-2">
              <input v-model="newLogDir"
                     type="text"
                     placeholder="选择或输入日志目录路径"
                     class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm"
                     readonly/>
              <button
                  class="px-4 py-2 cursor-pointer bg-blue-500 hover:bg-blue-600 text-white text-sm font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                  @click="selectLogDirectory">
                <Folder class="w-4 h-4"/>
              </button>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex gap-3 pt-2">
            <button
                @click="applyLogDirChange"
                :disabled="!newLogDir || newLogDir === currentLogDir"
                class="cursor-pointer px-4 py-2 bg-green-500 hover:bg-green-600 disabled:bg-gray-300 disabled:cursor-not-allowed text-white text-sm font-medium rounded-md transition-colors">
              应用更改
            </button>
            <button
                @click="openLogDirectory"
                class="cursor-pointer px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white text-sm font-medium rounded-md transition-colors">
              打开日志目录
            </button>
            <button
                @click="resetLogDirectory"
                class="cursor-pointer px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white text-sm font-medium rounded-md transition-colors">
              重置为默认
            </button>
          </div>

          <!-- 日志文件列表 -->
          <div v-if="logFiles.length > 0">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              最近的日志文件
            </label>
            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-3 max-h-32 overflow-y-auto">
              <div v-for="file in logFiles.slice(0, 5)" :key="file"
                   class="text-sm text-gray-600 dark:text-gray-400 font-mono py-1 hover:text-blue-600 dark:hover:text-blue-400 cursor-pointer"
                   @click="openLogFile(file)">
                {{ file }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 日志管理 -->
      <div class="mb-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
          <Settings2 class="w-5 h-5 mr-2"/>
          日志管理
        </h3>

        <div class="space-y-4">
          <!-- 清理旧日志 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              自动清理日志
            </label>
            <div class="flex items-center gap-3">
              <select v-model="keepDays"
                      class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm">
                <option value="7">保留 7 天</option>
                <option value="14">保留 14 天</option>
                <option value="30">保留 30 天</option>
                <option value="90">保留 90 天</option>
              </select>
              <button class="cursor-pointer px-4 py-2 bg-red-500 hover:bg-red-600 text-white text-sm font-medium rounded-md transition-colors"
                      @click="clearLogs">
                立即清理
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="flex justify-end gap-3 pt-4 border-t border-gray-200/50 dark:border-gray-600/50">
        <button
            @click="closeSettings"
            class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white text-sm font-medium rounded-md transition-colors">
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener'
import { FileText, Folder, Settings2, X } from 'lucide-vue-next'

const isVisible = ref(false)
const currentLogDir = ref('')
const newLogDir = ref('')
const logFiles = ref<string[]>([])
const keepDays = ref(30)

const emit = defineEmits<{
  close: []
}>()

const closeSettings = () => {
  isVisible.value = false
  setTimeout(() => {
    emit('close')
  }, 300)
}

const loadLogDirectory = async () => {
  try {
    const logDir = await invoke<string>('get_log_directory')
    currentLogDir.value = logDir
    newLogDir.value = logDir
  }
  catch (error) {
    console.error('Failed to get current log directory:', error)
  }
}

const loadLogFiles = async () => {
  try {
    const files = await invoke<string[]>('get_log_files')
    logFiles.value = files
  }
  catch (error) {
    console.error('Failed to get log files:', error)
  }
}

const selectLogDirectory = async () => {
  try {
    const selected = await openDialog({
      directory: true,
      multiple: false,
      title: '选择日志目录'
    })

    if (selected) {
      newLogDir.value = selected as string
    }
  }
  catch (error) {
    console.error('Failed to select directory:', error)
  }
}

const applyLogDirChange = async () => {
  try {
    await invoke('set_log_directory', { path: newLogDir.value })
    currentLogDir.value = newLogDir.value
    await loadLogFiles()
    // 这里可以显示成功提示
    console.log('日志目录已更新')
  }
  catch (error) {
    console.error('Failed to set log directory:', error)
    // 这里可以显示错误提示
  }
}

const openLogDirectory = async () => {
  try {
    await openPath(currentLogDir.value)
  }
  catch (error) {
    console.error('Failed to open log directory:', error)
  }
}

const resetLogDirectory = async () => {
  try {
    await invoke('reset_log_directory')
    await loadLogDirectory()
    await loadLogFiles()
    console.log('日志目录已重置为默认')
  }
  catch (error) {
    console.error('Failed to reset log directory:', error)
  }
}

const openLogFile = async (filename: string) => {
  try {
    const logPath = `${ currentLogDir.value }/${ filename }`
    await openPath(logPath)
  }
  catch (error) {
    console.error('Failed to open log file:', error)
  }
}

const clearLogs = async () => {
  try {
    await invoke('clear_logs', { keepDays: parseInt(keepDays.value.toString()) })
    await loadLogFiles()
    console.log(`已清理 ${ keepDays.value } 天前的日志`)
  }
  catch (error) {
    console.error('Failed to clear old logs:', error)
  }
}

onMounted(async () => {
  await loadLogDirectory()
  await loadLogFiles()

  // 延迟显示动画
  await nextTick()
  setTimeout(() => {
    isVisible.value = true
  }, 50)
})
</script>
