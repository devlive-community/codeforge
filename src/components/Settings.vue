<template>
  <Modal v-model:show="isVisible" title="设置" size="4xl" :close-on-backdrop="false" :close-on-esc="false" @close="closeSettings">
    <Tabs v-model="activeTab" type="card" size="md" :tabs="tabsData">
      <template #general>
        <General/>
      </template>
    </Tabs>
  </Modal>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from 'vue'
import { ShieldIcon } from 'lucide-vue-next'
import Modal from '../ui/Modal.vue'
import Tabs from '../ui/Tabs.vue'
import General from './setting/General.vue'

const isVisible = ref(false)
const activeTab = ref('general')
const tabsData = [
  { key: 'general', label: '通用', icon: ShieldIcon }
]

const emit = defineEmits<{
  close: []
}>()

const closeSettings = () => {
  isVisible.value = false
  setTimeout(() => {
    emit('close')
  }, 300)
}

onMounted(async () => {
  // 延迟显示动画
  await nextTick()
  setTimeout(() => {
    isVisible.value = true
  }, 50)
})
</script>
