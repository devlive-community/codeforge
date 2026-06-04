<template>
  <span class="relative inline-flex group">
    <slot/>
    <span v-if="text"
          class="pointer-events-none absolute z-50 whitespace-nowrap rounded bg-gray-800 px-2 py-1 text-xs text-white opacity-0 shadow-md transition-opacity duration-150 group-hover:opacity-100"
          :class="placementClass">
      {{ text }}
    </span>
  </span>
</template>

<script setup lang="ts">
import {computed} from 'vue'

const props = withDefaults(defineProps<{
  text: string
  placement?: 'top' | 'bottom'
}>(), {
  placement: 'bottom'
})

const placementClass = computed(() => props.placement === 'top'
    ? 'bottom-full mb-1.5 left-1/2 -translate-x-1/2'
    : 'top-full mt-1.5 left-1/2 -translate-x-1/2')
</script>
