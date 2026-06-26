<template>
  <div class="-mt-1">
    <p class="text-sm text-gray-600 dark:text-gray-300 mb-3">
      {{ t('settings.gitignore.desc') }}
    </p>

    <!-- 主从布局：左侧模板列表 + 右侧表单 -->
    <div class="flex gap-4 items-start">
      <!-- 左：模板列表 -->
      <div class="w-64 flex-shrink-0 space-y-2">
        <Button size="sm" :icon="Plus" class="w-full" :type="editingId ? 'secondary' : 'primary'" @click="resetForm">{{ t('settings.gitignore.add') }}</Button>
        <div class="space-y-1 max-h-[55vh] overflow-y-auto pr-0.5">
          <button v-for="tpl in list" :key="tpl.id"
                  class="group w-full text-left rounded-lg border px-2.5 py-2 transition-colors cursor-pointer"
                  :class="editingId === tpl.id ? 'border-blue-400 bg-blue-50 dark:border-blue-500 dark:bg-blue-900/20' : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800'"
                  @click="edit(tpl)">
            <div class="flex items-center gap-2">
              <span class="flex-1 truncate text-sm font-medium text-gray-800 dark:text-gray-100">{{ tpl.label }}</span>
              <Trash2 class="w-3.5 h-3.5 text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 flex-shrink-0" :title="t('settings.gitignore.delete')" @click.stop="remove(tpl.id)"/>
            </div>
            <div class="text-xs text-gray-400 truncate mt-0.5 font-mono">{{ preview(tpl.content) }}</div>
          </button>
          <div v-if="!list.length" class="text-xs text-gray-400 text-center py-6">{{ t('settings.gitignore.empty') }}</div>
        </div>
      </div>

      <!-- 右：表单 -->
      <div class="flex-1 min-w-0 border border-gray-200 dark:border-gray-700 rounded-lg p-4 space-y-4">
        <div class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ editingId ? t('settings.gitignore.editTitle') : t('settings.gitignore.newTitle') }}</div>

        <Label :label="t('settings.gitignore.name')">
          <Input v-model="label" :placeholder="t('settings.gitignore.labelPlaceholder')"/>
        </Label>
        <Label :label="t('settings.gitignore.content')">
          <textarea v-model="content" rows="10"
                    class="w-full text-xs font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"
                    :placeholder="t('settings.gitignore.contentPlaceholder')"/>
        </Label>

        <div class="flex items-center gap-2 pt-1">
          <Button size="sm" :disabled="!label.trim() || !content.trim()" @click="save">{{ editingId ? t('settings.gitignore.update') : t('settings.gitignore.add') }}</Button>
          <Button v-if="editingId" size="sm" type="secondary" @click="resetForm">{{ t('settings.gitignore.cancel') }}</Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Plus, Trash2} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import Input from '../../ui/Input.vue'
import Button from '../../ui/Button.vue'
import Label from '../../ui/Label.vue'
import {useToast} from '../../plugins/toast'

interface Tpl { id: string; label: string; content: string }

const {t} = useI18n()
const toast = useToast()
const list = ref<Tpl[]>([])
const label = ref('')
const content = ref('')
const editingId = ref('')

const preview = (c: string) => c.split('\n').filter(l => l.trim() && !l.trim().startsWith('#')).slice(0, 3).join('  ')

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
