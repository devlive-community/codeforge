<template>
  <div class="relative inline-block">
    <button class="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 cursor-pointer"
            @click.stop="open = !open">
      <Rocket class="w-3 h-3"/>
      {{ t('launch.presets') }}
      <ChevronDown class="w-3 h-3"/>
    </button>

    <template v-if="open">
      <div class="fixed inset-0 z-40" @click="open = false" @contextmenu.prevent="open = false"></div>
      <div class="absolute left-0 top-full mt-1 z-50 w-64 bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-xs">
        <!-- 保存当前 -->
        <div class="flex items-center gap-1.5 px-2 py-1.5 border-b border-gray-100 dark:border-gray-700">
          <input v-model="name"
                 class="flex-1 border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
                 :placeholder="t('launch.namePlaceholder')"
                 @keydown.enter="saveCurrent"/>
          <button class="px-2 py-1 rounded bg-brand-600 text-white hover:bg-brand-700 disabled:opacity-40 cursor-pointer whitespace-nowrap"
                  :disabled="!name.trim()"
                  @click="saveCurrent">
            {{ t('launch.save') }}
          </button>
        </div>
        <!-- 列表 -->
        <div class="max-h-52 overflow-y-auto">
          <div v-if="!presets.length" class="px-3 py-3 text-center text-gray-400">{{ t('launch.empty') }}</div>
          <div v-for="p in presets" :key="p.name"
               class="group flex items-center gap-1 px-2 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700">
            <button class="flex-1 min-w-0 text-left cursor-pointer" @click="applyPreset(p)">
              <div class="font-medium text-gray-800 dark:text-gray-100 truncate">{{ p.name }}</div>
              <div class="text-[10px] text-gray-400 truncate font-mono">{{ summary(p) }}</div>
            </button>
            <button class="p-1 rounded text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer"
                    :title="t('launch.delete')"
                    @click.stop="remove(p.name)">
              <Trash2 class="w-3 h-3"/>
            </button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {ChevronDown, Rocket, Trash2} from 'lucide-vue-next'
import {useLaunchPresets, type LaunchPreset} from '../composables/useLaunchPresets'

const {t} = useI18n()
const props = defineProps<{ args: string; stdin: string; env: string }>()
const emit = defineEmits<{ apply: [preset: LaunchPreset] }>()

const {presets, save, remove} = useLaunchPresets()
const open = ref(false)
const name = ref('')

const summary = (p: LaunchPreset) => [p.args, p.env].filter(Boolean).join(' · ') || t('launch.emptyInputs')

const saveCurrent = () => {
  if (!name.value.trim()) {
    return
  }
  save(name.value, props.args, props.stdin, props.env)
  name.value = ''
}

const applyPreset = (p: LaunchPreset) => {
  emit('apply', p)
  open.value = false
}
</script>
