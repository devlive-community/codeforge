<template>
  <div class="space-y-4">
    <div>
      <h3 class="text-sm font-medium text-gray-800 dark:text-gray-100">{{ t('settings.gitignore.title') }}</h3>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">{{ t('settings.gitignore.desc') }}</p>
    </div>

    <!-- 新增/编辑 -->
    <div class="rounded-lg border border-gray-200 dark:border-gray-700 p-3 space-y-2">
      <Input v-model="label" :placeholder="t('settings.gitignore.labelPlaceholder')"/>
      <textarea v-model="content" rows="6"
                class="w-full text-xs font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"
                :placeholder="t('settings.gitignore.contentPlaceholder')"/>
      <div class="flex justify-end gap-2">
        <Button v-if="editingId" size="sm" type="secondary" @click="resetForm">{{ t('settings.gitignore.cancel') }}</Button>
        <Button size="sm" :disabled="!label.trim() || !content.trim()" @click="save">{{ editingId ? t('settings.gitignore.update') : t('settings.gitignore.add') }}</Button>
      </div>
    </div>

    <!-- 列表 -->
    <div v-if="!list.length" class="text-xs text-gray-400 py-4 text-center">{{ t('settings.gitignore.empty') }}</div>
    <div v-for="tpl in list" :key="tpl.id" class="rounded-lg border border-gray-200 dark:border-gray-700 p-3">
      <div class="flex items-center gap-2">
        <div class="flex-1 min-w-0">
          <div class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ tpl.label }}</div>
          <div class="text-[11px] text-gray-400 font-mono truncate">{{ tpl.content.split('\n').filter(Boolean).slice(0, 1).join('') }}…</div>
        </div>
        <button class="text-xs text-blue-500 hover:underline cursor-pointer flex-shrink-0" @click="edit(tpl)">{{ t('settings.gitignore.edit') }}</button>
        <button class="text-xs text-red-500 hover:underline cursor-pointer flex-shrink-0" @click="remove(tpl.id)">{{ t('settings.gitignore.delete') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {useI18n} from 'vue-i18n'
import Input from '../../ui/Input.vue'
import Button from '../../ui/Button.vue'
import {useToast} from '../../plugins/toast'

interface Tpl { id: string; label: string; content: string }

const {t} = useI18n()
const toast = useToast()
const list = ref<Tpl[]>([])
const label = ref('')
const content = ref('')
const editingId = ref('')

const load = async () => {
  try {
    list.value = await invoke<Tpl[]>('gitignore_templates_list')
  }
  catch {
    list.value = []
  }
}

const resetForm = () => {
  label.value = ''
  content.value = ''
  editingId.value = ''
}

const save = async () => {
  if (!label.value.trim() || !content.value.trim()) {
    return
  }
  const id = editingId.value || crypto.randomUUID()
  try {
    await invoke('gitignore_template_save', {template: {id, label: label.value.trim(), content: content.value}})
    toast.success(t('settings.gitignore.saved'))
    resetForm()
    await load()
  }
  catch (e) {
    toast.error(t('settings.gitignore.failed') + ': ' + e)
  }
}

const edit = (tpl: Tpl) => {
  editingId.value = tpl.id
  label.value = tpl.label
  content.value = tpl.content
}

const remove = async (id: string) => {
  try {
    await invoke('gitignore_template_delete', {id})
    if (editingId.value === id) {
      resetForm()
    }
    await load()
  }
  catch (e) {
    toast.error(t('settings.gitignore.failed') + ': ' + e)
  }
}

onMounted(load)
</script>
