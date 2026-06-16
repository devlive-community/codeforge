<template>
  <div class="fixed top-0 right-0 bottom-0 z-40 w-[42%] min-w-[360px] max-w-[640px] bg-white dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700 shadow-2xl flex flex-col">
    <!-- 头部：分支 + 操作 -->
    <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200 min-w-0">
        <GitBranch class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <select v-if="status.is_repo"
                :value="status.branch"
                class="bg-transparent text-sm font-medium focus:outline-none cursor-pointer max-w-[180px] truncate dark:bg-gray-900"
                @change="onBranchChange">
          <option v-for="b in branches" :key="b" :value="b" class="dark:bg-gray-800">{{ b }}</option>
        </select>
        <span v-else class="text-gray-400">{{ t('git.notRepo') }}</span>
        <span v-if="status.ahead || status.behind" class="text-xs text-gray-400 flex-shrink-0">
          <span v-if="status.ahead">↑{{ status.ahead }}</span>
          <span v-if="status.behind">↓{{ status.behind }}</span>
        </span>
      </div>
      <div class="flex items-center gap-2 flex-shrink-0">
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.refresh')" @click="refresh">
          <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': loading }"/>
        </button>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.close')" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>
    </div>

    <template v-if="status.is_repo">
      <!-- 文件列表 -->
      <div class="flex-1 overflow-y-auto">
        <div v-if="staged.length" class="py-1">
          <div class="px-4 py-1 text-xs font-semibold text-gray-500 dark:text-gray-400 flex items-center justify-between">
            <span>{{ t('git.staged') }} ({{ staged.length }})</span>
            <button class="text-blue-500 hover:underline cursor-pointer" @click="unstageAll">{{ t('git.unstageAll') }}</button>
          </div>
          <FileRow v-for="f in staged" :key="'s' + f.path" :file="f" staged @toggle="unstage([f.path])" @open="openFile(f.path)"/>
        </div>

        <div v-if="unstaged.length" class="py-1">
          <div class="px-4 py-1 text-xs font-semibold text-gray-500 dark:text-gray-400 flex items-center justify-between">
            <span>{{ t('git.changes') }} ({{ unstaged.length }})</span>
            <button class="text-blue-500 hover:underline cursor-pointer" @click="stageAll">{{ t('git.stageAll') }}</button>
          </div>
          <FileRow v-for="f in unstaged" :key="'u' + f.path" :file="f" @toggle="stage([f.path])" @open="openFile(f.path)"/>
        </div>

        <div v-if="!staged.length && !unstaged.length" class="px-4 py-10 text-center text-sm text-gray-400">
          {{ t('git.clean') }}
        </div>
      </div>

      <!-- 提交区 -->
      <div class="border-t border-gray-200 dark:border-gray-700 p-3 flex-shrink-0 space-y-2">
        <div class="relative">
          <textarea v-model="message"
                    rows="3"
                    class="w-full text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 pr-9 focus:outline-none focus:border-blue-500 resize-none"
                    :placeholder="t('git.messagePlaceholder')"
                    @keydown.meta.enter.prevent="commit"
                    @keydown.ctrl.enter.prevent="commit"/>
          <button class="absolute top-1.5 right-1.5 p-1 rounded text-gray-400 hover:text-blue-500 hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer"
                  :disabled="generating"
                  :title="t('git.aiGenTitle')"
                  @click="genMessage">
            <Sparkles class="w-4 h-4" :class="{ 'animate-pulse': generating }"/>
          </button>
        </div>
        <div class="flex items-center gap-2">
          <Button size="sm" custom-class="[transform:translateZ(0)]" :loading="pending === 'commit'" :disabled="busy || !canCommit" @click="commit">
            {{ t('git.commit') }}{{ staged.length ? ` (${staged.length})` : '' }}
          </Button>
          <Button size="sm" type="secondary" custom-class="[transform:translateZ(0)]" :loading="pending === 'commitPush'" :disabled="busy" @click="commitAndPush">
            {{ t('git.commitPush') }}
          </Button>
          <Button size="sm" type="secondary" custom-class="[transform:translateZ(0)]" :loading="pending === 'push'" :disabled="busy" @click="push">
            {{ t('git.push') }}{{ status.ahead ? ` (↑${status.ahead})` : '' }}
          </Button>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, h, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {GitBranch, RefreshCw, Sparkles, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'
import {useAiConfig} from '../composables/useAiConfig'

interface GitFile { path: string; index: string; worktree: string }
interface GitStatusData { is_repo: boolean; branch: string; ahead: number; behind: number; files: GitFile[] }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: []; refresh: []; open: [path: string] }>()

const toast = useToast()
const {t} = useI18n()
const {active, reload: reloadAi} = useAiConfig()

const status = ref<GitStatusData>({is_repo: false, branch: '', ahead: 0, behind: 0, files: []})
const branches = ref<string[]>([])
const message = ref('')
const loading = ref(false)
// 正在进行的提交/推送动作，用于按钮加载状态；busy 据此派生
const pending = ref<'commit' | 'push' | 'commitPush' | null>(null)
const busy = computed(() => pending.value !== null)
const generating = ref(false)

// 文件视为已暂存：index 列非空且非未跟踪
const isStaged = (f: GitFile) => f.index !== ' ' && f.index !== '?'
// 同一文件可能同时存在暂存与未暂存改动；这里按是否有未暂存改动归入“更改”
const hasUnstaged = (f: GitFile) => f.worktree !== ' ' || f.index === '?'

const staged = computed(() => status.value.files.filter(isStaged))
const unstaged = computed(() => status.value.files.filter(hasUnstaged))

const canCommit = computed(() => staged.value.length > 0 && message.value.trim().length > 0 && !busy.value)

const refresh = async () => {
  loading.value = true
  try {
    status.value = await invoke<GitStatusData>('git_status', {root: props.rootDir})
    if (status.value.is_repo) {
      const b = await invoke<{ current: string; branches: string[] }>('git_branches', {root: props.rootDir})
      branches.value = b.branches
    }
    emit('refresh')
  }
  catch (error) {
    toast.error(t('git.statusFailed') + ': ' + error)
  }
  finally {
    loading.value = false
  }
}

const abs = (rel: string) => `${props.rootDir}/${rel}`

const stage = async (paths: string[]) => {
  try {
    await invoke('git_stage', {root: props.rootDir, paths: paths.map(abs)})
    await refresh()
  }
  catch (error) {
    toast.error(t('git.stageFailed') + ': ' + error)
  }
}
const unstage = async (paths: string[]) => {
  try {
    await invoke('git_unstage', {root: props.rootDir, paths: paths.map(abs)})
    await refresh()
  }
  catch (error) {
    toast.error(t('git.unstageFailed') + ': ' + error)
  }
}
const stageAll = () => stage(unstaged.value.map(f => f.path))
const unstageAll = () => unstage(staged.value.map(f => f.path))

// 仅执行 git 调用，不管 pending（供组合动作复用）
const doCommit = async () => {
  await invoke('git_commit', {root: props.rootDir, message: message.value.trim()})
  message.value = ''
}
const doPush = async () => {
  await invoke('git_push', {root: props.rootDir})
}

const commit = async () => {
  if (!canCommit.value) {
    return
  }
  pending.value = 'commit'
  try {
    await doCommit()
    toast.success(t('git.committed'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.commitFailed') + ': ' + error)
  }
  finally {
    pending.value = null
  }
}

const push = async () => {
  pending.value = 'push'
  try {
    await doPush()
    toast.success(t('git.pushed'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.pushFailed') + ': ' + error)
  }
  finally {
    pending.value = null
  }
}

const commitAndPush = async () => {
  if (!canCommit.value) {
    toast.info(t('git.needMessage'))
    return
  }
  pending.value = 'commitPush'
  try {
    await doCommit()
    await doPush()
    toast.success(t('git.committedPushed'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.commitPushFailed') + ': ' + error)
  }
  finally {
    pending.value = null
  }
}

const onBranchChange = async (e: Event) => {
  const branch = (e.target as HTMLSelectElement).value
  if (branch === status.value.branch) {
    return
  }
  try {
    await invoke('git_checkout', {root: props.rootDir, branch})
    toast.success(t('git.switched', { branch }))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.switchFailed') + ': ' + error)
    await refresh()
  }
}

const openFile = (rel: string) => emit('open', abs(rel))

// 清洗 AI 返回：去掉代码块/引号，取首个非空行
const cleanupMessage = (raw: string): string => {
  let s = raw.trim()
  s = s.replace(/^```[a-z]*\s*/i, '').replace(/```$/, '').trim()
  const first = s.split('\n').map(l => l.trim()).find(l => l.length > 0) || s
  return first.replace(/^["'「『]/, '').replace(/["'」』]$/, '').trim()
}

// 用 AI 根据当前改动生成提交信息
const genMessage = async () => {
  if (!status.value.is_repo) {
    return
  }
  reloadAi()
  if (!active.value.apiKey) {
    toast.info(t('git.aiNeedKey'))
    return
  }
  generating.value = true
  try {
    const diff = await invoke<string>('git_diff', {root: props.rootDir})
    if (!diff.trim()) {
      toast.info(t('git.noDiff'))
      return
    }
    const prompt = `根据下面的 git diff 生成一条简洁的中文提交信息，格式为「类型: 描述」（类型如 feat/fix/docs/refactor/chore），只输出一行提交信息，不要解释、不要代码块：\n\n${diff}`
    const res = await invoke<string>('ai_chat', {
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system: null,
      messages: [{role: 'user', content: prompt}]
    })
    message.value = cleanupMessage(res)
  }
  catch (error) {
    toast.error(t('git.genFailed') + ': ' + error)
  }
  finally {
    generating.value = false
  }
}

onMounted(refresh)

// 行内小组件：文件名 + 状态字母 + 暂存/取消按钮
const FileRow = (rowProps: { file: GitFile; staged?: boolean }, {emit: rowEmit }: any) => {
  const f = rowProps.file
  const code = f.index === '?' ? '?' : (rowProps.staged ? f.index : f.worktree).trim() || f.index.trim()
  const color = code === '?' ? 'text-green-500'
      : code === 'A' ? 'text-green-600 dark:text-green-400'
          : code === 'D' ? 'text-red-500'
              : code === 'M' ? 'text-amber-500'
                  : 'text-gray-400'
  const name = f.path.split('/').pop()
  const dir = f.path.slice(0, f.path.length - (name?.length || 0)).replace(/\/$/, '')
  return h('div', {class: 'group flex items-center px-4 py-1 hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer'}, [
    h('span', {
      class: 'flex-1 min-w-0 flex items-baseline gap-1.5',
      onClick: () => rowEmit('open')
    }, [
      h('span', {class: 'text-sm text-gray-800 dark:text-gray-200 truncate'}, name),
      h('span', {class: 'text-xs text-gray-400 truncate'}, dir)
    ]),
    h('span', {class: `text-xs font-bold w-4 text-center ${color}`}, code || 'M'),
    h('button', {
      class: 'ml-2 text-xs text-blue-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer',
      onClick: () => rowEmit('toggle')
    }, rowProps.staged ? '−' : '+')
  ])
}
</script>
