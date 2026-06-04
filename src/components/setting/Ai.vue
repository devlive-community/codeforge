<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2 flex items-center">
      <Sparkles class="w-5 h-5 mr-2"/>
      AI 助手
    </h3>

    <Label label="服务商">
      <Select v-model="state.provider" class="w-1/2" :options="providerOptions" @change="save"/>
    </Label>

    <Label label="API Key">
      <Input v-model="current.apiKey" type="password" class="w-full" placeholder="填写所选服务商的 API Key" @input="save"/>
    </Label>

    <Label label="模型">
      <Input v-model="current.model" class="w-full" :placeholder="activeMeta.defaultModel" @input="save"/>
    </Label>

    <Label label="接口地址（可选）">
      <Input v-model="current.baseUrl" class="w-full" :placeholder="activeMeta.defaultBase" @input="save"/>
    </Label>

    <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 text-sm text-blue-800 dark:text-blue-300">
      <ul class="space-y-1 list-disc list-inside">
        <li>请求经本地后端转发，避免浏览器跨域问题</li>
        <li>API Key 仅保存在本机</li>
        <li>留空接口地址则使用服务商默认地址</li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {Sparkles} from 'lucide-vue-next'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
import Select from '../../ui/Select.vue'
import {AI_PROVIDERS, useAiConfig} from '../../composables/useAiConfig'

const {state, save} = useAiConfig()

const providerOptions = AI_PROVIDERS.map(p => ({label: p.label, value: p.value}))

const current = computed(() => state.providers[state.provider])
const activeMeta = computed(() => AI_PROVIDERS.find(p => p.value === state.provider) || AI_PROVIDERS[0])
</script>
