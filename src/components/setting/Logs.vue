<template>
  <div>
    <!-- 日志设置 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <FileText class="w-5 h-5 mr-2"/>
        日志设置
      </h3>

      <div class="space-y-4">
        <Label label="当前日志目录">
          <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-3 text-sm text-gray-600 dark:text-gray-400 font-mono">
            {{ currentLogDir || '加载中...' }}
          </div>
        </Label>

        <Label label="选择新的日志目录">
          <div class="flex gap-2">
            <input v-model="newLogDir"
                   type="text"
                   placeholder="选择或输入日志目录路径"
                   class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm"
                   readonly/>

            <Button type="primary"
                    :icon-only="true"
                    :icon="Folder"
                    @click="selectLogDirectory">
            </Button>
          </div>
        </Label>

        <!-- 操作按钮 -->
        <div class="flex gap-3 pt-0.5">
          <Button @click="applyLogDirChange"
                  :disabled="!newLogDir || newLogDir === currentLogDir"
                  type="secondary">
            应用更改
          </Button>
          <Button @click="openLogDirectory" type="secondary">
            打开日志目录
          </Button>
          <Button @click="resetLogDirectory"
                  class="cursor-pointer px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white text-sm font-medium rounded-md transition-colors">
            重置为默认
          </Button>
        </div>

        <Label v-if="logFiles.length > 0" label="最近的日志文件">
          <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-3 max-h-32 overflow-y-auto">
            <div v-for="file in logFiles.slice(0, 5)" :key="file"
                 class="text-sm text-gray-600 dark:text-gray-400 font-mono py-1 hover:text-blue-600 dark:hover:text-blue-400 cursor-pointer"
                 @click="openLogFile(file)">
              {{ file }}
            </div>
          </div>
        </Label>
      </div>
    </div>

    <!-- 日志管理 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Settings2 class="w-5 h-5 mr-2"/>
        日志管理
      </h3>

      <div class="space-y-4">
        <Label label="清理日志">
          <div class="flex items-center gap-3">
            <Select v-model="keepDays"
                    class="w-36"
                    :options="keepDaysOptions"
                    placeholder="选择保留天数">
            </Select>
            <Button type="danger" @click="clearLogs">
              立即清理
            </Button>
          </div>
        </Label>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { FileText, Folder, Settings2 } from 'lucide-vue-next'
import Select from '../../ui/Select.vue'
import Button from '../../ui/Button.vue'
import Label from '../../ui/Label.vue'
import { useLogDirectory } from '../../composables/useLogDirectory'
import { useLogCleanup } from '../../composables/useLogCleanup'

const emit = defineEmits<{
  'settings-changed': [type: string, value: any]
  'error': [message: string]
}>()

const {
  currentLogDir,
  newLogDir,
  logFiles,
  loadLogDirectory,
  loadLogFiles,
  selectLogDirectory,
  applyLogDirChange,
  openLogDirectory,
  resetLogDirectory,
  openLogFile
} = useLogDirectory(emit)

const {
  keepDays,
  keepDaysOptions,
  clearLogs
} = useLogCleanup(emit, loadLogFiles)

defineExpose({
  loadLogDirectory,
  loadLogFiles,
  clearLogs
})

onMounted(async () => {
  await loadLogDirectory()
  await loadLogFiles()
})
</script>
