<template>
  <Teleport to="body">
    <div class="fixed z-[99999] bottom-4 right-4 space-y-2 w-96">
      <TransitionGroup name="toast"
                       enter-active-class="transition-all duration-300 ease-out"
                       enter-from-class="transform translate-x-full opacity-0"
                       enter-to-class="transform translate-x-0 opacity-100"
                       leave-active-class="transition-all duration-200 ease-in"
                       leave-from-class="transform translate-x-0 opacity-100"
                       leave-to-class="transform translate-x-full opacity-0">
        <div v-for="toast in toasts"
             :key="toast.id"
             class="max-w-sm w-full shadow-lg rounded-lg pointer-events-auto overflow-hidden"
             :class="getToastClasses(toast.type as string)">
          <div class="p-4">
            <div class="flex items-start">
              <!-- 图标 -->
              <div class="flex-shrink-0">
                <component :is="getIcon(toast.type as string)"
                           class="w-5 h-5"
                           :class="getIconClasses(toast.type as string)"/>
              </div>

              <!-- 消息内容 -->
              <div class="ml-3 w-0 flex-1 pt-0.5">
                <p class="text-sm font-medium text-gray-900">
                  {{ toast.message }}
                </p>
              </div>

              <!-- 关闭按钮 -->
              <div class="ml-4 flex-shrink-0 flex">
                <button @click="handleClose(toast.id)"
                        class="inline-flex cursor-pointer rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 transition-colors duration-200"
                        :class="getCloseButtonClasses(toast.type as string)">
                  <X class="w-4 h-4"/>
                </button>
              </div>
            </div>
          </div>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue'
import { AlertCircle, CheckCircle, Info, X, XCircle } from 'lucide-vue-next'
import { ToastManager } from '../plugins/toast'

// 注入 toast 管理器
const toastManager = inject('toast') as ToastManager

const toasts = computed(() => toastManager.getToasts())

const getIcon = (type: string) => {
  const iconMap = {
    success: CheckCircle,
    error: XCircle,
    warning: AlertCircle,
    info: Info
  }
  return iconMap[type as keyof typeof iconMap] || Info
}

const getToastClasses = (type: string) => {
  const baseClasses = 'bg-white border-l-4'
  const typeClasses = {
    success: 'border-green-500',
    error: 'border-red-500',
    warning: 'border-yellow-500',
    info: 'border-blue-500'
  }
  return `${ baseClasses } ${ typeClasses[type as keyof typeof typeClasses] || typeClasses.info }`
}

const getIconClasses = (type: string) => {
  const typeClasses = {
    success: 'text-green-500',
    error: 'text-red-500',
    warning: 'text-yellow-500',
    info: 'text-blue-500'
  }
  return typeClasses[type as keyof typeof typeClasses] || typeClasses.info
}

const getCloseButtonClasses = (type: string) => {
  const typeClasses = {
    success: 'text-green-400 hover:text-green-600 focus:ring-green-500',
    error: 'text-red-400 hover:text-red-600 focus:ring-red-500',
    warning: 'text-yellow-400 hover:text-yellow-600 focus:ring-yellow-500',
    info: 'text-blue-400 hover:text-blue-600 focus:ring-blue-500'
  }
  return typeClasses[type as keyof typeof typeClasses] || typeClasses.info
}

const handleClose = (id: number) => {
  toastManager.remove(id)
}
</script>
