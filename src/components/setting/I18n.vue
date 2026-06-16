<template>
  <div class="space-y-4 max-w-3xl">
    <p class="text-sm text-gray-600 dark:text-gray-400">{{ t('settings.i18n.desc') }}</p>

    <!-- 编辑视图 -->
    <div v-if="editingCode" class="space-y-3">
      <div class="flex items-center gap-2">
        <span class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ editingCode }}</span>
        <Input v-model="editName" class="w-48" :placeholder="t('settings.i18n.name')"/>
        <span class="text-[10px] px-1.5 py-0.5 rounded uppercase font-semibold"
              :class="builtinEditing ? 'bg-blue-100 dark:bg-blue-900/40 text-blue-600 dark:text-blue-300' : 'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-600 dark:text-emerald-300'">
          {{ builtinEditing ? t('settings.i18n.builtin') : t('settings.i18n.custom') }}
        </span>
      </div>
      <div>
        <div class="text-[11px] text-gray-400 mb-1">{{ t('settings.i18n.messages') }}</div>
        <textarea v-model="editJson"
                  rows="18"
                  spellcheck="false"
                  class="w-full text-xs font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500 resize-y"/>
      </div>
      <div class="flex items-center gap-2">
        <Button size="sm" @click="save">{{ t('settings.i18n.save') }}</Button>
        <Button v-if="builtinEditing" size="sm" type="secondary" @click="reset">{{ t('settings.i18n.resetBuiltin') }}</Button>
        <Button size="sm" type="secondary" @click="editingCode = null">{{ t('settings.i18n.cancel') }}</Button>
      </div>
    </div>

    <!-- 列表视图 -->
    <template v-else>
      <div class="border border-gray-200 dark:border-gray-700 rounded-lg divide-y divide-gray-100 dark:divide-gray-700 overflow-hidden">
        <div v-for="l in availableLocales" :key="l.value" class="flex items-center gap-3 px-3 py-2">
          <span class="text-[10px] px-1.5 py-0.5 rounded uppercase font-semibold flex-shrink-0"
                :class="l.builtin ? 'bg-blue-100 dark:bg-blue-900/40 text-blue-600 dark:text-blue-300' : 'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-600 dark:text-emerald-300'">
            {{ l.builtin ? t('settings.i18n.builtin') : t('settings.i18n.custom') }}
          </span>
          <div class="flex-1 min-w-0">
            <span class="text-sm font-medium text-gray-800 dark:text-gray-100">{{ l.label }}</span>
            <span class="ml-2 text-xs text-gray-400">{{ l.value }}</span>
          </div>
          <button class="text-xs text-blue-500 hover:underline cursor-pointer" @click="startEdit(l.value)">{{ t('settings.i18n.edit') }}</button>
          <button v-if="!l.builtin" class="text-xs text-red-500 hover:underline cursor-pointer" @click="remove(l.value)">{{ t('settings.i18n.delete') }}</button>
        </div>
      </div>

      <!-- 新增语言 -->
      <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4 space-y-3">
        <div class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ t('settings.i18n.addLocale') }}</div>
        <div class="grid grid-cols-2 gap-x-4 gap-y-3">
          <Label :label="t('settings.i18n.code')">
            <Input v-model="newCode" :placeholder="t('settings.i18n.codePlaceholder')"/>
          </Label>
          <Label :label="t('settings.i18n.name')">
            <Input v-model="newName" :placeholder="t('settings.i18n.namePlaceholder')"/>
          </Label>
          <Label :label="t('settings.i18n.basedOn')" custom-class="col-span-2">
            <Select v-model="newBase" :options="baseOptions" class="w-full" :button-classes="['!py-1.5', 'text-sm', 'w-full']"/>
          </Label>
        </div>
        <Button size="sm" @click="createNew">{{ t('settings.i18n.create') }}</Button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import Button from '../../ui/Button.vue'
import Input from '../../ui/Input.vue'
import Select from '../../ui/Select.vue'
import Label from '../../ui/Label.vue'
import {useToast} from '../../plugins/toast'
import {
  availableLocales, isBuiltinLocale, getLocaleMessages, getBuiltinMessages,
  saveLocale, deleteLocale, resetBuiltin
} from '../../i18n'

const {t} = useI18n()
const toast = useToast()

const editingCode = ref<string | null>(null)
const editName = ref('')
const editJson = ref('')
const builtinEditing = computed(() => editingCode.value ? isBuiltinLocale(editingCode.value) : false)

const newCode = ref('')
const newName = ref('')
const newBase = ref('zh-CN')
const baseOptions = computed(() => availableLocales.value.map(l => ({value: l.value, label: l.label})))

const startEdit = (code: string) => {
  editingCode.value = code
  editName.value = availableLocales.value.find(l => l.value === code)?.label || code
  editJson.value = JSON.stringify(getLocaleMessages(code), null, 2)
}

const save = () => {
  let parsed: Record<string, any>
  try {
    parsed = JSON.parse(editJson.value)
  }
  catch {
    toast.error(t('settings.i18n.invalidJson'))
    return
  }
  saveLocale(editingCode.value!, editName.value.trim() || editingCode.value!, parsed)
  toast.success(t('settings.i18n.saved'))
  editingCode.value = null
}

const reset = () => {
  if (!editingCode.value) {
    return
  }
  resetBuiltin(editingCode.value)
  toast.success(t('settings.i18n.resetDone'))
  editingCode.value = null
}

const remove = (code: string) => {
  deleteLocale(code)
  toast.success(t('settings.i18n.deleted'))
}

const createNew = () => {
  const code = newCode.value.trim()
  const name = newName.value.trim()
  if (!code || !name) {
    toast.error(t('settings.i18n.codeRequired'))
    return
  }
  if (availableLocales.value.some(l => l.value === code)) {
    toast.error(t('settings.i18n.codeExists'))
    return
  }
  // 以所选语言的现有文案为模板
  const base = JSON.parse(JSON.stringify(getLocaleMessages(newBase.value) || getBuiltinMessages('zh-CN')))
  saveLocale(code, name, base)
  newCode.value = ''
  newName.value = ''
  startEdit(code)
}
</script>
