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
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer flex-shrink-0" :title="t('git.branchMenu')" @click="branchMenu = !branchMenu">
          <GitBranchPlus class="w-3.5 h-3.5"/>
        </button>
      </div>

      <!-- 分支管理弹层 -->
      <template v-if="branchMenu">
        <div class="fixed inset-0 z-40" @click="branchMenu = false"/>
        <div class="absolute left-3 top-12 z-50 w-64 rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg p-2 text-sm">
          <div class="flex gap-1 mb-2">
            <input v-model="newBranchName"
                   class="flex-1 min-w-0 text-xs border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
                   :placeholder="t('git.newBranchPlaceholder')"
                   @keydown.enter="createBranch"/>
            <button class="px-2 py-1 text-xs rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 cursor-pointer flex-shrink-0"
                    :disabled="!newBranchName.trim()" @click="createBranch">{{ t('git.create') }}</button>
          </div>
          <div class="max-h-60 overflow-y-auto">
            <div v-for="b in branches" :key="b"
                 class="group flex items-center gap-1 px-1.5 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700">
              <span class="flex-1 truncate cursor-pointer"
                    :class="b === status.branch ? 'font-semibold text-blue-600 dark:text-blue-400' : 'text-gray-700 dark:text-gray-200'"
                    @click="switchBranch(b)">{{ b }}</span>
              <button class="p-0.5 text-gray-400 hover:text-blue-500 opacity-0 group-hover:opacity-100 cursor-pointer" :title="t('git.renameBranch')" @click="openRenameBranch(b)">
                <Pencil class="w-3.5 h-3.5"/>
              </button>
              <template v-if="b !== status.branch">
                <button class="p-0.5 text-gray-400 hover:text-emerald-500 opacity-0 group-hover:opacity-100 cursor-pointer" :title="t('git.mergeInto')" @click="mergeBranch(b)">
                  <GitMerge class="w-3.5 h-3.5"/>
                </button>
                <button class="p-0.5 text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer" :title="t('git.deleteBranch')" @click="deleteBranch(b)">
                  <Trash2 class="w-3.5 h-3.5"/>
                </button>
              </template>
            </div>
            <template v-if="remoteBranches.length">
              <div class="mt-1.5 mb-1 text-[11px] text-gray-400">{{ t('git.remoteBranches') }}</div>
              <div v-for="rb in remoteBranches" :key="rb"
                   class="group flex items-center gap-1 px-1.5 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700">
                <span class="flex-1 truncate text-gray-500 dark:text-gray-400">{{ rb }}</span>
                <button class="p-0.5 text-gray-400 hover:text-emerald-500 opacity-0 group-hover:opacity-100 cursor-pointer" :title="t('git.checkoutTrack')" @click="checkoutTrack(rb)">
                  <GitBranchPlus class="w-3.5 h-3.5"/>
                </button>
              </div>
            </template>
          </div>
        </div>
      </template>
      <div class="flex items-center gap-2 flex-shrink-0">
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.remoteTitle')" @click="showRemotes = true">
          <Cloud class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.tagTitle')" @click="showTags = true">
          <Tag class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.identityTitle')" @click="showConfig = true">
          <UserCog class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.stashTitle')" @click="showStash = true">
          <Archive class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.history')" @click="showLog = true">
          <History class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.graph')" @click="showGraph = true">
          <Network class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo && submoduleCount > 0" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.submodule')" @click="showSubmodules = true">
          <Boxes class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.worktree')" @click="showWorktrees = true">
          <TreeDeciduous class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.bisect')" @click="showBisect = true">
          <Crosshair class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.compare')" @click="showCompare = true">
          <GitCompareArrows class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.reflog')" @click="showReflog = true">
          <RotateCcw class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-red-500 cursor-pointer" :title="t('git.clean')" @click="openClean">
          <Eraser class="w-4 h-4"/>
        </button>
        <button v-if="status.is_repo" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer disabled:opacity-40" :title="t('git.fetch')" :disabled="busy" @click="fetch">
          <DownloadCloud class="w-4 h-4" :class="{ 'animate-pulse': pending === 'fetch' }"/>
        </button>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.refresh')" @click="refresh">
          <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': loading }"/>
        </button>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('git.close')" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>
    </div>

    <template v-if="status.is_repo">
      <!-- 进行中操作横幅（merge/rebase/cherry-pick/revert）-->
      <div v-if="opState !== 'none'" class="flex items-center gap-2 px-4 py-2 bg-amber-50 dark:bg-amber-900/20 border-b border-amber-200 dark:border-amber-800 flex-shrink-0">
        <AlertTriangle class="w-4 h-4 text-amber-500 flex-shrink-0"/>
        <span class="flex-1 min-w-0 text-xs text-amber-700 dark:text-amber-300">{{ t('git.opInProgress', { op: opState }) }}</span>
        <button class="text-xs text-emerald-600 dark:text-emerald-400 hover:underline cursor-pointer flex-shrink-0" @click="opAction('continue')">{{ t('git.opContinue') }}</button>
        <button v-if="opState !== 'merge'" class="text-xs text-blue-500 hover:underline cursor-pointer flex-shrink-0" @click="opAction('skip')">{{ t('git.opSkip') }}</button>
        <button class="text-xs text-red-500 hover:underline cursor-pointer flex-shrink-0" @click="opAction('abort')">{{ t('git.opAbort') }}</button>
      </div>

      <!-- 文件列表 -->
      <div class="flex-1 overflow-y-auto">
        <div v-if="conflicts.length" class="py-1">
          <div class="px-4 py-1 text-xs font-semibold text-red-500 flex items-center gap-1">
            <AlertTriangle class="w-3.5 h-3.5"/>{{ t('git.conflicts') }} ({{ conflicts.length }})
          </div>
          <div v-for="f in conflicts" :key="'c' + f.path" class="group flex items-center px-4 py-1 hover:bg-gray-100 dark:hover:bg-gray-800">
            <span class="flex-1 min-w-0 text-sm text-gray-800 dark:text-gray-200 truncate cursor-pointer" @click="openFile(f.path)">{{ f.path }}</span>
            <button class="text-xs text-blue-500 hover:underline cursor-pointer flex-shrink-0" @click="resolve(f.path)">{{ t('git.resolve') }}</button>
          </div>
        </div>

        <div v-if="staged.length" class="py-1">
          <div class="px-4 py-1 text-xs font-semibold text-gray-500 dark:text-gray-400 flex items-center justify-between">
            <span>{{ t('git.staged') }} ({{ staged.length }})</span>
            <button class="text-blue-500 hover:underline cursor-pointer" @click="unstageAll">{{ t('git.unstageAll') }}</button>
          </div>
          <FileRow v-for="f in staged" :key="'s' + f.path" :file="f" staged @toggle="unstage([f.path])" @open="openFile(f.path)" @diff="viewDiff(f.path)" @discard="requestDiscard(f)" @hunks="openHunks(f.path, true)"/>
        </div>

        <div v-if="unstaged.length" class="py-1">
          <div class="px-4 py-1 text-xs font-semibold text-gray-500 dark:text-gray-400 flex items-center justify-between">
            <span>{{ t('git.changes') }} ({{ unstaged.length }})</span>
            <button class="text-blue-500 hover:underline cursor-pointer" @click="stageAll">{{ t('git.stageAll') }}</button>
          </div>
          <FileRow v-for="f in unstaged" :key="'u' + f.path" :file="f" @toggle="stage([f.path])" @open="openFile(f.path)" @diff="viewDiff(f.path)" @discard="requestDiscard(f)" @hunks="openHunks(f.path, false)"/>
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
        <div class="flex flex-wrap items-center gap-x-3 gap-y-1">
          <label class="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400 cursor-pointer select-none">
            <input v-model="amend" type="checkbox" class="cursor-pointer"/>
            {{ t('git.amend') }}
          </label>
          <label class="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400 cursor-pointer select-none">
            <input v-model="commitAll" type="checkbox" class="cursor-pointer"/>
            {{ t('git.commitAll') }}
          </label>
          <label class="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400 cursor-pointer select-none">
            <input v-model="signoff" type="checkbox" class="cursor-pointer"/>
            {{ t('git.signoff') }}
          </label>
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
          <Button size="sm" type="secondary" custom-class="[transform:translateZ(0)]" :loading="pending === 'pull'" :disabled="busy" @click="pull">
            {{ t('git.pull') }}{{ status.behind ? ` (↓${status.behind})` : '' }}
          </Button>
          <div class="relative">
            <button class="p-1.5 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('git.more')" @click="moreMenu = !moreMenu">
              <MoreHorizontal class="w-4 h-4"/>
            </button>
            <template v-if="moreMenu">
              <div class="fixed inset-0 z-40" @click="moreMenu = false"/>
              <div class="absolute right-0 bottom-full mb-1 z-50 w-52 py-1 rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-sm">
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :disabled="busy" @click="moreAction('pullRebase')">{{ t('git.pullRebase') }}</button>
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :disabled="busy" @click="moreAction('pushTags')">{{ t('git.pushTags') }}</button>
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-amber-600 dark:text-amber-400" :disabled="busy" @click="moreAction('forcePush')">{{ t('git.forcePush') }}</button>
              </div>
            </template>
          </div>
        </div>
      </div>
    </template>
  </div>

  <!-- 丢弃改动确认 -->
  <Modal v-model:show="showDiscard" :title="t('git.discardTitle')" size="sm">
    <div class="space-y-4">
      <p class="text-sm text-gray-700 dark:text-gray-300">
        {{ t('git.discardConfirm', { file: discardTarget?.path }) }}
      </p>
      <div class="flex justify-end gap-2">
        <Button size="sm" type="secondary" @click="showDiscard = false">{{ t('git.cancel') }}</Button>
        <Button size="sm" type="danger" @click="confirmDiscard">{{ t('git.discard') }}</Button>
      </div>
    </div>
  </Modal>

  <!-- 清理未跟踪文件确认 -->
  <Modal v-model:show="cleanModal.show" :title="t('git.cleanTitle')" size="sm">
    <div class="space-y-3">
      <template v-if="cleanModal.list.length">
        <p class="text-sm text-red-500">{{ t('git.cleanConfirm') }}</p>
        <div class="max-h-48 overflow-y-auto rounded border border-gray-200 dark:border-gray-700 p-2 text-xs font-mono text-gray-600 dark:text-gray-300">
          <div v-for="p in cleanModal.list" :key="p" class="truncate">{{ p }}</div>
        </div>
        <div class="flex justify-end gap-2">
          <Button size="sm" type="secondary" @click="cleanModal.show = false">{{ t('git.cancel') }}</Button>
          <Button size="sm" type="danger" @click="confirmClean">{{ t('git.clean') }}</Button>
        </div>
      </template>
      <template v-else>
        <p class="text-sm text-gray-500 dark:text-gray-400">{{ t('git.cleanEmpty') }}</p>
        <div class="flex justify-end">
          <Button size="sm" type="secondary" @click="cleanModal.show = false">{{ t('git.close') }}</Button>
        </div>
      </template>
    </div>
  </Modal>

  <!-- 重命名分支 -->
  <Modal v-model:show="renameBranch.show" :title="t('git.renameBranchTitle')" size="sm">
    <div class="space-y-4">
      <input v-model="renameBranch.value"
             class="w-full text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"
             @keydown.enter="confirmRenameBranch"/>
      <div class="flex justify-end gap-2">
        <Button size="sm" type="secondary" @click="renameBranch.show = false">{{ t('git.cancel') }}</Button>
        <Button size="sm" :disabled="!renameBranch.value.trim()" @click="confirmRenameBranch">{{ t('git.renameBranch') }}</Button>
      </div>
    </div>
  </Modal>

  <!-- 储藏 -->
  <GitStash v-if="showStash" :root-dir="rootDir" @close="showStash = false" @changed="refresh"/>

  <!-- 标签 -->
  <GitTags v-if="showTags" :root-dir="rootDir" @close="showTags = false"/>

  <!-- 远程 -->
  <GitRemotes v-if="showRemotes" :root-dir="rootDir" :branch="status.branch" @close="showRemotes = false"/>

  <!-- 仓库身份 -->
  <GitConfig v-if="showConfig" :root-dir="rootDir" @close="showConfig = false"/>

  <!-- 分块暂存 -->
  <HunkStageView v-if="hunkFile" :root-dir="rootDir" :rel-path="hunkFile.path" :staged="hunkFile.staged" @close="hunkFile = null" @changed="refresh"/>

  <!-- 分支图 -->
  <GitGraph v-if="showGraph" :root-dir="rootDir" @close="showGraph = false"/>

  <!-- 子模块 -->
  <GitSubmodules v-if="showSubmodules" :root-dir="rootDir" @close="showSubmodules = false"/>

  <!-- 工作树 -->
  <GitWorktrees v-if="showWorktrees" :root-dir="rootDir" @close="showWorktrees = false"/>

  <!-- 二分定位 -->
  <GitBisect v-if="showBisect" :root-dir="rootDir" @close="showBisect = false" @changed="refresh"/>

  <!-- 提交历史 -->
  <GitLog v-if="showLog" :root-dir="rootDir" @close="showLog = false" @changed="refresh"/>

  <!-- 引用日志 reflog -->
  <GitReflog v-if="showReflog" :root-dir="rootDir" @close="showReflog = false" @changed="refresh"/>

  <!-- 分支对比 -->
  <GitCompare v-if="showCompare" :root-dir="rootDir" :branch="status.branch" @close="showCompare = false"/>

  <!-- 单文件改动对比：HEAD vs 工作区 -->
  <DiffView v-if="diffFile"
            :original="diffFile.original"
            :modified="diffFile.modified"
            :file-name="diffFile.name"
            :title="t('git.diffTitle')"
            :subtitle="t('git.diffSubtitle')"
            @close="diffFile = null"/>
</template>

<script setup lang="ts">
import {computed, h, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {AlertTriangle, Archive, Boxes, Cloud, Crosshair, DownloadCloud, Eraser, GitBranch, GitBranchPlus, GitCompare as GitCompareIcon, GitCompareArrows, GitMerge, History, MoreHorizontal, Network, Pencil, RefreshCw, RotateCcw, Rows3, Sparkles, Tag, Trash2, TreeDeciduous, Undo2, UserCog, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Modal from '../ui/Modal.vue'
import DiffView from './DiffView.vue'
import GitLog from './GitLog.vue'
import GitReflog from './GitReflog.vue'
import GitCompare from './GitCompare.vue'
import GitStash from './GitStash.vue'
import GitTags from './GitTags.vue'
import GitRemotes from './GitRemotes.vue'
import GitConfig from './GitConfig.vue'
import HunkStageView from './HunkStageView.vue'
import GitGraph from './GitGraph.vue'
import GitSubmodules from './GitSubmodules.vue'
import GitWorktrees from './GitWorktrees.vue'
import GitBisect from './GitBisect.vue'
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
// 进行中操作：none / merge / rebase / cherry-pick / revert
const opState = ref('none')
const branches = ref<string[]>([])
const remoteBranches = ref<string[]>([])
const message = ref('')
const loading = ref(false)
// 正在进行的提交/推送动作，用于按钮加载状态；busy 据此派生
const pending = ref<'commit' | 'push' | 'commitPush' | 'pull' | 'fetch' | null>(null)
const busy = computed(() => pending.value !== null)
const generating = ref(false)
// 提交选项
const amend = ref(false)
const commitAll = ref(false)
const signoff = ref(false)
// 单文件改动对比（HEAD vs 工作区）
const diffFile = ref<{ name: string; original: string; modified: string } | null>(null)
// 丢弃改动确认
const showDiscard = ref(false)
const discardTarget = ref<GitFile | null>(null)
// 提交历史 / 储藏 / 标签
const showLog = ref(false)
const showReflog = ref(false)
const showCompare = ref(false)
// 更多推送/拉取选项
const moreMenu = ref(false)
const showStash = ref(false)
const showTags = ref(false)
const showRemotes = ref(false)
const showConfig = ref(false)
const showGraph = ref(false)
const showSubmodules = ref(false)
const submoduleCount = ref(0)
const showWorktrees = ref(false)
const showBisect = ref(false)
const hunkFile = ref<{ path: string; staged: boolean } | null>(null)
const openHunks = (path: string, staged: boolean) => {
  hunkFile.value = {path, staged}
}
// 分支管理弹层
const branchMenu = ref(false)
const newBranchName = ref('')
const renameBranch = ref<{ show: boolean; old: string; value: string }>({show: false, old: '', value: ''})
// 清理未跟踪文件
const cleanModal = ref<{ show: boolean; list: string[] }>({show: false, list: []})

// 未合并（冲突）：任一侧为 U，或两侧同为 A/D（AA/DD）
const isConflict = (f: GitFile) => f.index === 'U' || f.worktree === 'U' || (f.index === f.worktree && (f.index === 'A' || f.index === 'D'))
// 文件视为已暂存：index 列非空且非未跟踪
const isStaged = (f: GitFile) => f.index !== ' ' && f.index !== '?'
// 同一文件可能同时存在暂存与未暂存改动；这里按是否有未暂存改动归入“更改”
const hasUnstaged = (f: GitFile) => f.worktree !== ' ' || f.index === '?'

const conflicts = computed(() => status.value.files.filter(isConflict))
const staged = computed(() => status.value.files.filter(f => isStaged(f) && !isConflict(f)))
const unstaged = computed(() => status.value.files.filter(f => hasUnstaged(f) && !isConflict(f)))

// 普通提交需暂存+信息；amend 或 -a 时放宽暂存要求
const canCommit = computed(() => !busy.value
    && (amend.value || commitAll.value || staged.value.length > 0)
    && (amend.value || message.value.trim().length > 0))

const refresh = async () => {
  loading.value = true
  try {
    status.value = await invoke<GitStatusData>('git_status', {root: props.rootDir})
    if (status.value.is_repo) {
      const b = await invoke<{ current: string; branches: string[] }>('git_branches', {root: props.rootDir})
      branches.value = b.branches
      remoteBranches.value = await invoke<string[]>('git_remote_branches', {root: props.rootDir})
      opState.value = await invoke<string>('git_op_state', {root: props.rootDir})
      submoduleCount.value = (await invoke<unknown[]>('git_submodules', {root: props.rootDir})).length
    }
    else {
      opState.value = 'none'
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
// 标记冲突已解决：git add 该文件
const resolve = async (path: string) => {
  try {
    await invoke('git_stage', {root: props.rootDir, paths: [abs(path)]})
    toast.success(t('git.resolved'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.stageFailed') + ': ' + error)
  }
}
const stageAll = () => stage(unstaged.value.map(f => f.path))
const unstageAll = () => unstage(staged.value.map(f => f.path))

// 仅执行 git 调用，不管 pending（供组合动作复用）
const doCommit = async () => {
  await invoke('git_commit', {root: props.rootDir, message: message.value.trim(), amend: amend.value, all: commitAll.value, signoff: signoff.value})
  message.value = ''
  amend.value = false
  commitAll.value = false
  signoff.value = false
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

// 进行中操作的 继续/中止/跳过
const opAction = async (action: 'continue' | 'abort' | 'skip') => {
  const op = opState.value
  if (op === 'none') {
    return
  }
  const cmd = action === 'continue' ? 'git_op_continue' : action === 'abort' ? 'git_op_abort' : 'git_op_skip'
  const msg = action === 'continue' ? 'opContinued' : action === 'abort' ? 'opAborted' : 'opSkipped'
  try {
    await invoke(cmd, {root: props.rootDir, op})
    toast.success(t('git.' + msg))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.opFailed') + ': ' + error)
    await refresh()
  }
}

const openClean = async () => {
  try {
    const list = await invoke<string[]>('git_clean_preview', {root: props.rootDir})
    cleanModal.value = {show: true, list}
  }
  catch (error) {
    toast.error(t('git.cleanFailed') + ': ' + error)
  }
}
const confirmClean = async () => {
  cleanModal.value.show = false
  try {
    await invoke('git_clean', {root: props.rootDir})
    toast.success(t('git.cleaned'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.cleanFailed') + ': ' + error)
  }
}

const pull = async () => {
  pending.value = 'pull'
  try {
    await invoke('git_pull', {root: props.rootDir})
    toast.success(t('git.pulled'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.pullFailed') + ': ' + error)
  }
  finally {
    pending.value = null
  }
}

const moreAction = async (action: 'pullRebase' | 'pushTags' | 'forcePush') => {
  moreMenu.value = false
  const map = {
    pullRebase: {cmd: 'git_pull_rebase', msg: 'pullRebased'},
    pushTags: {cmd: 'git_push_tags', msg: 'tagsPushed'},
    forcePush: {cmd: 'git_push_force', msg: 'forcePushed'}
  } as const
  const {cmd, msg} = map[action]
  pending.value = 'push'
  try {
    await invoke(cmd, {root: props.rootDir})
    toast.success(t('git.' + msg))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.pushFailed') + ': ' + error)
  }
  finally {
    pending.value = null
  }
}

const fetch = async () => {
  pending.value = 'fetch'
  try {
    await invoke('git_fetch', {root: props.rootDir})
    toast.success(t('git.fetched'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.fetchFailed') + ': ' + error)
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

const switchBranch = async (branch: string) => {
  branchMenu.value = false
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

const checkoutTrack = async (remoteBranch: string) => {
  branchMenu.value = false
  try {
    await invoke('git_checkout_track', {root: props.rootDir, remoteBranch})
    toast.success(t('git.tracked'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.switchFailed') + ': ' + error)
    await refresh()
  }
}

const createBranch = async () => {
  const name = newBranchName.value.trim()
  if (!name) {
    return
  }
  try {
    await invoke('git_branch_create', {root: props.rootDir, name})
    newBranchName.value = ''
    branchMenu.value = false
    toast.success(t('git.branchCreated'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.branchOpFailed') + ': ' + error)
  }
}

const deleteBranch = async (name: string) => {
  try {
    await invoke('git_branch_delete', {root: props.rootDir, name})
    toast.success(t('git.branchDeleted'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.branchOpFailed') + ': ' + error)
  }
}

const openRenameBranch = (b: string) => {
  branchMenu.value = false
  renameBranch.value = {show: true, old: b, value: b}
}
const confirmRenameBranch = async () => {
  const {old, value} = renameBranch.value
  const next = value.trim()
  if (!next || next === old) {
    renameBranch.value.show = false
    return
  }
  try {
    await invoke('git_branch_rename', {root: props.rootDir, old, new: next})
    renameBranch.value.show = false
    toast.success(t('git.branchRenamed'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.branchOpFailed') + ': ' + error)
  }
}

const mergeBranch = async (branch: string) => {
  branchMenu.value = false
  try {
    await invoke('git_merge', {root: props.rootDir, branch})
    toast.success(t('git.merged'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.branchOpFailed') + ': ' + error)
    await refresh()
  }
}

const openFile = (rel: string) => emit('open', abs(rel))

// 查看某文件相对 HEAD 的改动：取 HEAD 内容与工作区内容交给 DiffView
const viewDiff = async (rel: string) => {
  try {
    const head = await invoke<{ exists: boolean; content: string }>('git_file_head', {root: props.rootDir, relPath: rel})
    let work = ''
    try {
      work = await invoke<string>('read_file_text', {path: abs(rel)})
    }
    catch {
      // 文件已删除或为二进制：工作区内容按空处理
      work = ''
    }
    diffFile.value = {name: rel, original: head.exists ? head.content : '', modified: work}
  }
  catch (error) {
    toast.error(t('git.diffFailed') + ': ' + error)
  }
}

const requestDiscard = (f: GitFile) => {
  discardTarget.value = f
  showDiscard.value = true
}

const confirmDiscard = async () => {
  const f = discardTarget.value
  showDiscard.value = false
  if (!f) {
    return
  }
  try {
    if (f.index === '?') {
      // 未跟踪文件：丢弃即删除
      await invoke('delete_path', {path: abs(f.path)})
    }
    else {
      await invoke('git_discard', {root: props.rootDir, paths: [abs(f.path)]})
    }
    toast.success(t('git.discarded'))
    await refresh()
  }
  catch (error) {
    toast.error(t('git.discardFailed') + ': ' + error)
  }
}

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
      class: 'ml-2 text-gray-400 hover:text-blue-500 opacity-0 group-hover:opacity-100 cursor-pointer',
      title: t('git.viewDiff'),
      onClick: () => rowEmit('diff')
    }, h(GitCompareIcon, {class: 'w-3.5 h-3.5'})),
    code === 'M' ? h('button', {
      class: 'ml-1.5 text-gray-400 hover:text-blue-500 opacity-0 group-hover:opacity-100 cursor-pointer',
      title: t('git.hunks'),
      onClick: () => rowEmit('hunks')
    }, h(Rows3, {class: 'w-3.5 h-3.5'})) : null,
    h('button', {
      class: 'ml-1.5 text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer',
      title: t('git.discard'),
      onClick: () => rowEmit('discard')
    }, h(Undo2, {class: 'w-3.5 h-3.5'})),
    h('button', {
      class: 'ml-1.5 text-xs text-blue-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer',
      onClick: () => rowEmit('toggle')
    }, rowProps.staged ? '−' : '+')
  ])
}
</script>
