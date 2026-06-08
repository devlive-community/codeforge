<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h3 class="text-sm font-medium text-gray-800 dark:text-gray-100">语言服务（LSP）</h3>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">安装后可获得精准补全、悬浮文档、跳转定义、查找引用、重命名与实时诊断</p>
      </div>
      <button class="inline-flex items-center gap-1 px-2 py-1 text-xs rounded border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="reload">
        <RefreshCw class="w-3.5 h-3.5" :class="loading ? 'animate-spin' : ''"/>重新检测
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
          <span v-if="s.installed" class="text-xs text-emerald-600 dark:text-emerald-400 flex-shrink-0">已安装</span>
          <button v-else class="px-2.5 py-1 text-xs rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50 cursor-pointer flex-shrink-0"
                  :disabled="installing === s.id" @click="install(s)">
            <RefreshCw v-if="installing === s.id" class="w-3 h-3 animate-spin inline"/>
            {{ installing === s.id ? '安装中…' : '安装' }}
          </button>
        </div>
        <!-- 安装日志 -->
        <pre v-if="logs[s.id]" class="mt-2 max-h-32 overflow-auto rounded bg-gray-950 text-gray-300 text-[11px] p-2 whitespace-pre-wrap">{{ logs[s.id] }}</pre>
      </div>
    </div>

    <p class="text-[11px] text-gray-400 leading-relaxed">
      自动安装依赖本机已具备对应工具链：npm（pyright / typescript / intelephense / vscode-langservers-extracted）、rustup（rust-analyzer）、go（gopls）、Homebrew（clangd / lua-language-server）、gem（solargraph）。
      若安装失败，可按上方命令手动安装后点「重新检测」。安装完成后请重启应用以加载新服务器。
    </p>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {RefreshCw} from 'lucide-vue-next'

interface ServerInfo { id: string; label: string; program: string; installed: boolean; install: string }

const servers = ref<ServerInfo[]>([])
const loading = ref(false)
const installing = ref('')
const logs = ref<Record<string, string>>({})
let unlisten: UnlistenFn[] = []

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
}

const install = async (s: ServerInfo) => {
  installing.value = s.id
  logs.value = {...logs.value, [s.id]: '开始安装…\n'}
  try {
    await invoke('lsp_install', {id: s.id})
  }
  catch (e: any) {
    logs.value = {...logs.value, [s.id]: (logs.value[s.id] || '') + '启动安装失败：' + String(e?.message || e) + '\n'}
    installing.value = ''
  }
}

onMounted(async () => {
  await reload()
  unlisten.push(await listen<[string, string]>('lsp:install', (e) => {
    const [id, line] = e.payload
    logs.value = {...logs.value, [id]: (logs.value[id] || '') + line + '\n'}
  }))
  unlisten.push(await listen<[string, boolean]>('lsp:install-done', async (e) => {
    const [id, success] = e.payload
    logs.value = {...logs.value, [id]: (logs.value[id] || '') + (success ? '\n✓ 安装完成' : '\n✗ 安装失败') + '\n'}
    installing.value = ''
    await reload()
  }))
})
onUnmounted(() => {
  for (const u of unlisten) {
    u()
  }
  unlisten = []
})
</script>
