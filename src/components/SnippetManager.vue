<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[760px] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[80vh]"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Code2 class="w-4 h-4 text-gray-400"/>
          <span>{{ t('snippet.title') }}</span>
          <span class="text-xs text-gray-400">· {{ t('snippet.subtitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('snippet.close')" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto">
        <!-- 列表 -->
        <div v-if="snippets.length" class="divide-y divide-gray-100 dark:divide-gray-700">
          <div v-for="s in snippets" :key="s.id" class="px-4 py-2 flex items-start gap-3 hover:bg-gray-50 dark:hover:bg-gray-900/40">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <code class="text-sm font-semibold text-blue-600 dark:text-blue-400">{{ s.prefix }}</code>
                <span class="text-[10px] px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400">{{ s.language && s.language !== '*' ? s.language : t('snippet.allLanguages') }}</span>
                <span v-if="s.description" class="text-xs text-gray-400 truncate">{{ s.description }}</span>
              </div>
              <pre class="mt-1 text-xs text-gray-500 dark:text-gray-400 font-mono whitespace-pre-wrap line-clamp-3">{{ s.body }}</pre>
            </div>
            <div class="flex items-center gap-1 flex-shrink-0">
              <button class="p-1 rounded text-gray-400 hover:text-blue-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('snippet.edit')" @click="startEdit(s)">
                <Pencil class="w-3.5 h-3.5"/>
              </button>
              <button class="p-1 rounded text-gray-400 hover:text-red-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('snippet.delete')" @click="remove(s.id)">
                <Trash2 class="w-3.5 h-3.5"/>
              </button>
            </div>
          </div>
        </div>
        <div v-else class="px-4 py-8 text-center text-sm text-gray-400">{{ t('snippet.empty') }}</div>
      </div>

      <!-- 编辑/新增表单 -->
      <div class="border-t border-gray-200 dark:border-gray-700 p-3 flex-shrink-0 space-y-2">
        <div class="flex gap-2">
          <input v-model="form.prefix" :placeholder="t('snippet.prefixPlaceholder')" class="w-40 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1 focus:outline-none focus:border-blue-500"/>
          <input v-model="form.language" :placeholder="t('snippet.langPlaceholder')" class="w-44 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1 focus:outline-none focus:border-blue-500"/>
          <input v-model="form.description" :placeholder="t('snippet.descPlaceholder')" class="flex-1 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1 focus:outline-none focus:border-blue-500"/>
        </div>
        <textarea v-model="form.body" rows="4" :placeholder="t('snippet.bodyPlaceholder')" class="w-full text-sm font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1.5 resize-none focus:outline-none focus:border-blue-500"/>
        <div class="flex items-center gap-2">
          <Button size="sm" :disabled="!canSave" @click="submit">{{ editingId ? t('snippet.saveEdit') : t('snippet.addSnippet') }}</Button>
          <Button v-if="editingId" size="sm" type="secondary" @click="resetForm">{{ t('snippet.cancelEdit') }}</Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, reactive, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Code2, Pencil, Trash2, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useSnippets, type Snippet} from '../composables/useSnippets'

const emit = defineEmits<{ close: [] }>()
const {t} = useI18n()

const {snippets, add, update, remove} = useSnippets()

const editingId = ref<string | null>(null)
const form = reactive<{ prefix: string; language: string; description: string; body: string }>({
  prefix: '', language: '', description: '', body: ''
})

const canSave = computed(() => form.prefix.trim().length > 0 && form.body.length > 0)

const resetForm = () => {
  editingId.value = null
  form.prefix = ''
  form.language = ''
  form.description = ''
  form.body = ''
}

const startEdit = (s: Snippet) => {
  editingId.value = s.id
  form.prefix = s.prefix
  form.language = s.language && s.language !== '*' ? s.language : ''
  form.description = s.description || ''
  form.body = s.body
}

const submit = () => {
  if (!canSave.value) {
    return
  }
  const payload = {
    prefix: form.prefix.trim(),
    language: form.language.trim() || '*',
    description: form.description.trim(),
    body: form.body
  }
  if (editingId.value) {
    update(editingId.value, payload)
  }
  else {
    add(payload)
  }
  resetForm()
}
</script>
