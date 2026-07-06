<template>
  <div class="fixed inset-0 z-[55] flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[760px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
      <!-- 标题栏 + 选项卡 -->
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-3 text-sm font-medium text-gray-700 dark:text-gray-200 min-w-0">
          <Github class="w-4 h-4 text-gray-400 flex-shrink-0"/>
          <span class="truncate">{{ owner }}/{{ repo }}</span>
          <div class="flex items-center gap-1 text-xs">
            <button class="px-2 py-0.5 rounded cursor-pointer" :class="tab === 'pr' ? 'bg-blue-500 text-white' : 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700'" @click="switchTab('pr')">{{ t('gh.prs') }}</button>
            <button class="px-2 py-0.5 rounded cursor-pointer" :class="tab === 'issue' ? 'bg-blue-500 text-white' : 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700'" @click="switchTab('issue')">{{ t('gh.issues') }}</button>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('gh.refresh')" @click="reload">
            <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': loading }"/>
          </button>
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
            <X class="w-4 h-4"/>
          </button>
        </div>
      </div>

      <!-- 无 token 提示 -->
      <div v-if="!token" class="flex-1 flex items-center justify-center text-center text-sm text-gray-400 px-6">
        {{ t('gh.needToken') }}
      </div>

      <template v-else>
        <!-- 新建表单（可折叠） -->
        <div class="border-b border-gray-100 dark:border-gray-800 flex-shrink-0">
          <button class="w-full flex items-center gap-1.5 px-4 py-2 text-xs text-blue-500 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer" @click="showForm = !showForm">
            <component :is="showForm ? ChevronDown : ChevronRight" class="w-3.5 h-3.5"/>
            {{ tab === 'pr' ? t('gh.newPr') : t('gh.newIssue') }}
          </button>
          <div v-if="showForm" class="px-4 pb-3 flex flex-col gap-2">
            <input v-model="form.title" :placeholder="t('gh.titlePlaceholder')"
                   class="w-full text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-1.5 focus:outline-none focus:ring-1 focus:ring-blue-400"/>
            <div v-if="tab === 'pr'" class="flex items-center gap-2 text-xs">
              <span class="text-gray-400">{{ t('gh.from') }}</span>
              <span class="px-2 py-1 rounded bg-gray-100 dark:bg-gray-800 font-mono">{{ branch }}</span>
              <span class="text-gray-400">→</span>
              <input v-model="form.base" placeholder="main"
                     class="w-32 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 font-mono focus:outline-none"/>
              <label class="flex items-center gap-1 cursor-pointer ml-2"><input type="checkbox" v-model="form.draft" class="cursor-pointer"/>{{ t('gh.draft') }}</label>
            </div>
            <textarea v-model="form.body" rows="3" :placeholder="t('gh.bodyPlaceholder')"
                      class="w-full text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-1.5 resize-none focus:outline-none focus:ring-1 focus:ring-blue-400"></textarea>
            <div class="flex justify-end">
              <button class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer"
                      :disabled="!form.title.trim() || submitting" @click="submit">
                {{ submitting ? t('gh.submitting') : (tab === 'pr' ? t('gh.createPr') : t('gh.createIssue')) }}
              </button>
            </div>
          </div>
        </div>

        <!-- 列表 -->
        <div class="flex-1 overflow-auto">
          <div v-if="loading" class="p-6 text-center text-sm text-gray-400">{{ t('gh.loading') }}</div>
          <div v-else-if="error" class="p-6 text-center text-sm text-red-500">{{ error }}</div>
          <div v-else-if="items.length === 0" class="p-6 text-center text-sm text-gray-400">{{ tab === 'pr' ? t('gh.noPrs') : t('gh.noIssues') }}</div>
          <div v-for="it in items" :key="it.number"
               class="flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
               @click="openUrl(it.url)">
            <component :is="tab === 'pr' ? GitPullRequest : CircleDot" class="w-4 h-4 flex-shrink-0" :class="it.draft ? 'text-gray-400' : 'text-green-500'"/>
            <div class="min-w-0 flex-1">
              <div class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ it.title }}</div>
              <div class="text-[11px] text-gray-400 truncate">
                #{{ it.number }} · {{ it.author }}
                <template v-if="tab === 'pr'"> · <span class="font-mono">{{ it.head }} → {{ it.base }}</span><span v-if="it.draft"> · {{ t('gh.draft') }}</span></template>
                <template v-else-if="it.comments"> · 💬 {{ it.comments }}</template>
              </div>
            </div>
            <ExternalLink class="w-3.5 h-3.5 text-gray-300 flex-shrink-0"/>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, reactive, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {open as openExternalUrl} from '@tauri-apps/plugin-shell'
import {ChevronDown, ChevronRight, CircleDot, ExternalLink, Github, GitPullRequest, RefreshCw, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useToast} from '../plugins/toast'

const props = defineProps<{ owner: string; repo: string; branch: string }>()
const emit = defineEmits<{ close: [] }>()

const {t} = useI18n()
const toast = useToast()

const tab = ref<'pr' | 'issue'>('pr')
const token = ref('')
const loading = ref(false)
const error = ref('')
const items = ref<any[]>([])
const showForm = ref(false)
const submitting = ref(false)
const form = reactive({title: '', base: 'main', body: '', draft: false})

const openUrl = (url: string) => url && openExternalUrl(url)

const reload = async () => {
  if (!token.value) {
    return
  }
  loading.value = true
  error.value = ''
  try {
    items.value = tab.value === 'pr'
      ? await invoke<any[]>('github_list_prs', {token: token.value, owner: props.owner, repo: props.repo})
      : await invoke<any[]>('github_list_issues', {token: token.value, owner: props.owner, repo: props.repo})
  }
  catch (e: any) {
    error.value = String(e?.message || e)
  }
  finally {
    loading.value = false
  }
}

const switchTab = (tb: 'pr' | 'issue') => {
  if (tab.value === tb) {
    return
  }
  tab.value = tb
  items.value = []
  reload()
}

const submit = async () => {
  if (!form.title.trim()) {
    return
  }
  submitting.value = true
  try {
    if (tab.value === 'pr') {
      const pr = await invoke<any>('github_create_pr', {
        token: token.value, owner: props.owner, repo: props.repo,
        title: form.title.trim(), head: props.branch, base: form.base.trim() || 'main',
        body: form.body, draft: form.draft
      })
      items.value = [pr, ...items.value]
      toast.success(t('gh.prCreated', {n: pr.number}))
    }
    else {
      const issue = await invoke<any>('github_create_issue', {
        token: token.value, owner: props.owner, repo: props.repo,
        title: form.title.trim(), body: form.body
      })
      items.value = [issue, ...items.value]
      toast.success(t('gh.issueCreated', {n: issue.number}))
    }
    form.title = ''
    form.body = ''
    form.draft = false
    showForm.value = false
  }
  catch (e: any) {
    toast.error(String(e?.message || e))
  }
  finally {
    submitting.value = false
  }
}

onMounted(async () => {
  try {
    const config = await invoke<any>('get_app_config')
    token.value = config?.github?.token || ''
  }
  catch { /* 忽略 */ }
  reload()
})
</script>
