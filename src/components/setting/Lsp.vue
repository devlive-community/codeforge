<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h3 class="text-sm font-medium text-gray-800 dark:text-gray-100">{{ t('settings.lsp.title') }}</h3>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">{{ t('settings.lsp.desc') }}</p>
      </div>
      <button class="inline-flex items-center gap-1 px-2 py-1 text-xs rounded border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="reload">
        <RefreshCw class="w-3.5 h-3.5" :class="loading ? 'animate-spin' : ''"/>{{ t('settings.lsp.recheck') }}
      </button>
    </div>

    <div class="space-y-2">
      <div v-for="s in servers" :key="s.id" class="rounded-lg border border-gray-200 dark:border-gray-700 p-3">
        <div class="flex items-center gap-3">
          <span class="w-2 h-2 rounded-full flex-shrink-0" :class="s.installed ? 'bg-emerald-500' : 'bg-gray-300 dark:bg-gray-600'"/>
          <div class="min-w-0 flex-1">
            <div class="text-sm text-gray-800 dark:text-gray-100">{{ s.label }}</div>
            <code class="text-[11px] text-gray-400">{{ s.install }}</code>
          </div>
          <span v-if="s.installed" class="text-xs text-emerald-600 dark:text-emerald-400 flex-shrink-0">{{ t('settings.lsp.installed') }}</span>
          <button v-else class="px-2.5 py-1 text-xs rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50 cursor-pointer flex-shrink-0"
                  :disabled="installing === s.id" @click="install(s)">
            <RefreshCw v-if="installing === s.id" class="w-3 h-3 animate-spin inline"/>
            {{ installing === s.id ? t('settings.lsp.installing') : t('settings.lsp.install') }}
          </button>
        </div>
        <!-- 安装日志 -->
        <pre v-if="logs[s.id]" class="mt-2 max-h-32 overflow-auto rounded bg-gray-950 text-gray-300 text-[11px] p-2 whitespace-pre-wrap">{{ logs[s.id] }}</pre>
      </div>
    </div>

    <p class="text-[11px] text-gray-400 leading-relaxed">
      {{ t('settings.lsp.note') }}
    </p>

    <!-- 调试适配器 -->
    <div class="pt-2 border-t border-gray-200 dark:border-gray-700">
      <h3 class="text-sm font-medium text-gray-800 dark:text-gray-100">{{ t('settings.lsp.dapTitle') }}</h3>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5 mb-2">{{ t('settings.lsp.dapDesc') }}</p>
      <div class="space-y-2">
        <div v-for="a in adapters" :key="a.id" class="rounded-lg border border-gray-200 dark:border-gray-700 p-3">
          <div class="flex items-center gap-3">
            <span class="w-2 h-2 rounded-full flex-shrink-0" :class="a.installed ? 'bg-emerald-500' : 'bg-gray-300 dark:bg-gray-600'"/>
            <div class="min-w-0 flex-1">
              <div class="text-sm text-gray-800 dark:text-gray-100">{{ a.label }}</div>
              <code class="text-[11px] text-gray-400">{{ a.install }}</code>
            </div>
            <span v-if="a.installed" class="text-xs text-emerald-600 dark:text-emerald-400 flex-shrink-0">{{ t('settings.lsp.installed') }}</span>
            <button v-else class="px-2.5 py-1 text-xs rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50 cursor-pointer flex-shrink-0"
                    :disabled="installingDap === a.id" @click="installDap(a)">
              <RefreshCw v-if="installingDap === a.id" class="w-3 h-3 animate-spin inline"/>
              {{ installingDap === a.id ? t('settings.lsp.installing') : t('settings.lsp.install') }}
            </button>
          </div>
          <pre v-if="dapLogs[a.id]" class="mt-2 max-h-32 overflow-auto rounded bg-gray-950 text-gray-300 text-[11px] p-2 whitespace-pre-wrap">{{ dapLogs[a.id] }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {RefreshCw} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'

const {t} = useI18n()

interface ServerInfo { id: string; label: string; program: string; installed: boolean; install: string }
interface AdapterInfo { id: string; label: string; installed: boolean; install: string }

const servers = ref<ServerInfo[]>([])
const loading = ref(false)
const installing = ref('')
const logs = ref<Record<string, string>>({})
const adapters = ref<AdapterInfo[]>([])
const installingDap = ref('')
const dapLogs = ref<Record<string, string>>({})
let unlisten: UnlistenFn[] = []

const reloadDap = async () => {
  try {
    adapters.value = await invoke<AdapterInfo[]>('dap_adapter_list')
  }
  catch {
    adapters.value = []
  }
}

const installDap = async (a: AdapterInfo) => {
  installingDap.value = a.id
  dapLogs.value = {...dapLogs.value, [a.id]: t('settings.lsp.startInstall') + '\n'}
  try {
    await invoke('dap_install', {id: a.id})
  }
  catch (e: any) {
    dapLogs.value = {...dapLogs.value, [a.id]: (dapLogs.value[a.id] || '') + t('settings.lsp.startInstallFailed') + String(e?.message || e) + '\n'}
    installingDap.value = ''
  }
}

const reload = async () => {
  loading.value = true
  try {
    servers.value = await invoke<ServerInfo[]>('lsp_server_list')
  }
  catch {
    servers.value = []
  }
  finally {
    loading.value = false
  }
  await reloadDap()
}

const install = async (s: ServerInfo) => {
  installing.value = s.id
  logs.value = {...logs.value, [s.id]: t('settings.lsp.startInstall') + '\n'}
  try {
    await invoke('lsp_install', {id: s.id})
  }
  catch (e: any) {
    logs.value = {...logs.value, [s.id]: (logs.value[s.id] || '') + t('settings.lsp.startInstallFailed') + String(e?.message || e) + '\n'}
    installing.value = ''
  }
}

onMounted(async () => {
  await reload()
  await reloadDap()
  unlisten.push(await listen<[string, string]>('lsp:install', (e) => {
    const [id, line] = e.payload
    logs.value = {...logs.value, [id]: (logs.value[id] || '') + line + '\n'}
  }))
  unlisten.push(await listen<[string, boolean]>('lsp:install-done', async (e) => {
    const [id, success] = e.payload
    logs.value = {...logs.value, [id]: (logs.value[id] || '') + (success ? '\n' + t('settings.lsp.installDone') : '\n' + t('settings.lsp.installFailed')) + '\n'}
    installing.value = ''
    await reload()
  }))
  unlisten.push(await listen<[string, string]>('dap:install', (e) => {
    const [id, line] = e.payload
    dapLogs.value = {...dapLogs.value, [id]: (dapLogs.value[id] || '') + line + '\n'}
  }))
  unlisten.push(await listen<[string, boolean]>('dap:install-done', async (e) => {
    const [id, success] = e.payload
    dapLogs.value = {...dapLogs.value, [id]: (dapLogs.value[id] || '') + (success ? '\n' + t('settings.lsp.installDone') : '\n' + t('settings.lsp.installFailed')) + '\n'}
    installingDap.value = ''
    await reloadDap()
  }))
})
onUnmounted(() => {
  for (const u of unlisten) {
    u()
  }
  unlisten = []
})
</script>
