<template>
  <div class="flex items-stretch bg-gray-50 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 overflow-x-auto flex-shrink-0
              [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
    <div v-for="tab in tabs"
         :key="tab.id"
         class="group flex items-center space-x-2 pl-3 pr-2 py-1.5 border-r border-gray-200 dark:border-gray-700 cursor-pointer max-w-[200px] flex-shrink-0 transition-colors"
         :class="[
           tab.id === activeId ? 'bg-white dark:bg-gray-900 text-gray-800 dark:text-gray-100' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700',
           dragOverId === tab.id ? 'border-l-2 border-l-blue-500' : ''
         ]"
         draggable="true"
         @click="emit('switch', tab.id)"
         @mousedown.middle.prevent="emit('close', tab.id)"
         @contextmenu.prevent.stop="openMenu(tab, $event)"
         @dragstart="onDragStart(tab.id, $event)"
         @dragover.prevent="dragOverId = tab.id"
         @dragleave="dragOverId === tab.id && (dragOverId = null)"
         @drop.prevent="onDrop(tab.id)"
         @dragend="onDragEnd">
      <img :src="iconUrl(tab.language)" class="w-4 h-4 flex-shrink-0" :alt="tab.language"
           @error="(e) => { const t = e.target as HTMLImageElement; if (!t.src.endsWith('/icons/text.svg')) t.src = '/icons/text.svg' }"/>
      <span class="text-xs truncate">{{ title(tab) }}</span>
      <span v-if="isDirty(tab) && !tab.pinned" class="text-amber-500 text-xs flex-shrink-0" :title="t('tabs.unsaved')">●</span>
      <!-- 置顶图标：默认显示，hover 时让位给关闭按钮 -->
      <button v-if="tab.pinned"
              class="ml-1 rounded p-0.5 text-brand-500 group-hover:hidden flex-shrink-0"
              :title="t('tabs.unpin')"
              @click.stop="emit('toggle-pin', tab.id)">
        <Pin class="w-3 h-3 fill-current"/>
      </button>
      <button class="ml-1 rounded p-0.5 text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 transition-opacity flex-shrink-0"
              :class="tab.pinned ? 'hidden group-hover:inline-flex' : (tab.id === activeId ? 'opacity-100' : 'opacity-0 group-hover:opacity-100')"
              :title="t('tabs.close')"
              @click.stop="emit('close', tab.id)">
        <X class="w-3 h-3"/>
      </button>
    </div>

    <button class="flex items-center justify-center px-2 text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer flex-shrink-0"
            :title="t('tabs.newTab')"
            @click="emit('new')">
      <Plus class="w-4 h-4"/>
    </button>
  </div>

  <!-- 标签右键菜单 -->
  <div v-if="menu.visible" class="fixed inset-0 z-50" @click="closeMenu" @contextmenu.prevent="closeMenu">
    <div class="absolute bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-sm min-w-[140px]"
         :style="{ top: `${menu.y}px`, left: `${menu.x}px` }"
         @click.stop>
      <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('toggle-pin', menu.tabId!))">{{ menuTabPinned ? t('tabs.unpin') : t('tabs.pin') }}</button>
      <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
      <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('close', menu.tabId!))">{{ t('tabs.close') }}</button>
      <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('close-others', menu.tabId!))">{{ t('tabs.closeOthers') }}</button>
      <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('close-right', menu.tabId!))">{{ t('tabs.closeRight') }}</button>
      <template v-if="menuTabPath">
        <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('reveal-tree', menuTabPath!))">{{ t('tabs.revealInTree') }}</button>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('reveal-finder', menuTabPath!))">{{ t('tabs.revealInFinder') }}</button>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('copy-path', menuTabPath!))">{{ t('tabs.copyPath') }}</button>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="act(() => emit('copy-relative', menuTabPath!))">{{ t('tabs.copyRelativePath') }}</button>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, reactive, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Pin, Plus, X} from 'lucide-vue-next'
import type {WorkspaceTab} from '../composables/useWorkspace'

const {t} = useI18n()

const props = defineProps<{
  tabs: WorkspaceTab[]
  activeId: string
}>()

const emit = defineEmits<{
  switch: [id: string]
  close: [id: string]
  new: []
  'close-others': [id: string]
  'close-right': [id: string]
  'copy-path': [path: string]
  'copy-relative': [path: string]
  'reveal-tree': [path: string]
  'reveal-finder': [path: string]
  'toggle-pin': [id: string]
  move: [fromId: string, toId: string]
}>()

const title = (tab: WorkspaceTab) => {
  if (!tab.filePath) {
    return t('tabs.untitled')
  }
  return tab.filePath.split(/[\\/]/).pop() || t('tabs.untitled')
}

const isDirty = (tab: WorkspaceTab) => tab.filePath !== null && tab.code !== tab.savedContent

const iconUrl = (language: string) => `/icons/${language.replace(/\d+$/, '')}.svg`

// ===== 拖拽排序 =====
const draggingId = ref<string | null>(null)
const dragOverId = ref<string | null>(null)

const onDragStart = (id: string, e: DragEvent) => {
  draggingId.value = id
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move'
    // 部分 webview 需要写入数据才会真正启动拖拽
    e.dataTransfer.setData('text/plain', id)
  }
}
const onDrop = (id: string) => {
  if (draggingId.value && draggingId.value !== id) {
    emit('move', draggingId.value, id)
  }
  dragOverId.value = null
}
const onDragEnd = () => {
  draggingId.value = null
  dragOverId.value = null
}

// ===== 右键菜单 =====
const menu = reactive<{ visible: boolean, x: number, y: number, tabId: string | null }>({
  visible: false, x: 0, y: 0, tabId: null
})
const menuTabPath = computed(() => props.tabs.find(t => t.id === menu.tabId)?.filePath || null)
const menuTabPinned = computed(() => props.tabs.find(t => t.id === menu.tabId)?.pinned || false)
const openMenu = (tab: WorkspaceTab, e: MouseEvent) => {
  menu.tabId = tab.id
  menu.x = e.clientX
  menu.y = e.clientY
  menu.visible = true
}
const closeMenu = () => {
  menu.visible = false
}
const act = (fn: () => void) => {
  fn()
  closeMenu()
}
</script>
