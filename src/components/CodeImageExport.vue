<template>
  <div class="fixed inset-0 z-[55] flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[860px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
      <!-- 标题栏 -->
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <ImageIcon class="w-4 h-4 text-gray-400"/>
          <span>{{ t('codeImage.title') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 选项 -->
      <div class="flex items-center flex-wrap gap-x-5 gap-y-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800 flex-shrink-0 text-xs">
        <div class="flex items-center gap-1.5">
          <span class="text-gray-400">{{ t('codeImage.theme') }}</span>
          <button v-for="(th, i) in THEMES" :key="i"
                  class="w-5 h-5 rounded-full border-2 cursor-pointer"
                  :class="themeIdx === i ? 'border-blue-500' : 'border-transparent'"
                  :style="{ background: th.swatch }" :title="th.name" @click="themeIdx = i"></button>
        </div>
        <div class="flex items-center gap-1.5">
          <span class="text-gray-400">{{ t('codeImage.padding') }}</span>
          <select v-model.number="padding" class="rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-0.5 focus:outline-none cursor-pointer">
            <option v-for="p in [16, 32, 48, 64]" :key="p" :value="p">{{ p }}</option>
          </select>
        </div>
        <label class="flex items-center gap-1 cursor-pointer text-gray-500 dark:text-gray-400">
          <input type="checkbox" v-model="showChrome" class="cursor-pointer"/>{{ t('codeImage.windowChrome') }}
        </label>
        <label class="flex items-center gap-1 cursor-pointer text-gray-500 dark:text-gray-400">
          <input type="checkbox" v-model="showLineNumbers" class="cursor-pointer"/>{{ t('codeImage.lineNumbers') }}
        </label>
      </div>

      <!-- 预览 -->
      <div class="flex-1 overflow-auto p-6 flex items-start justify-center bg-gray-100 dark:bg-gray-950">
        <div ref="cardRef" v-html="cardHtml"></div>
      </div>

      <!-- 底部 -->
      <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
        <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-40" :disabled="busy" @click="copyImage">{{ t('codeImage.copy') }}</button>
        <button class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 cursor-pointer disabled:opacity-40" :disabled="busy" @click="downloadImage">{{ busy ? t('codeImage.rendering') : t('codeImage.download') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {Image as ImageIcon, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useToast} from '../plugins/toast'

const props = defineProps<{ code: string; language: string; fileName?: string }>()
const emit = defineEmits<{ close: [] }>()

const {t} = useI18n()
const toast = useToast()

const THEMES = [
  {name: 'Indigo', bg: 'linear-gradient(135deg,#667eea 0%,#764ba2 100%)', swatch: 'linear-gradient(135deg,#667eea,#764ba2)'},
  {name: 'Sunset', bg: 'linear-gradient(135deg,#ff9a9e 0%,#fad0c4 100%)', swatch: 'linear-gradient(135deg,#ff9a9e,#fad0c4)'},
  {name: 'Ocean', bg: 'linear-gradient(135deg,#2193b0 0%,#6dd5ed 100%)', swatch: 'linear-gradient(135deg,#2193b0,#6dd5ed)'},
  {name: 'Mint', bg: 'linear-gradient(135deg,#43e97b 0%,#38f9d7 100%)', swatch: 'linear-gradient(135deg,#43e97b,#38f9d7)'},
  {name: 'Graphite', bg: '#0d1117', swatch: '#0d1117'}
]
const FONT = "ui-monospace,'SF Mono',Menlo,Monaco,Consolas,'Courier New',monospace"

const themeIdx = ref(0)
const padding = ref(32)
const showChrome = ref(true)
const showLineNumbers = ref(false)
const busy = ref(false)
const cardRef = ref<HTMLElement>()

const escapeHtml = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

// 行号列（作为独立列，避免破坏代码对齐）
const codeInner = computed(() => {
  const esc = escapeHtml(props.code.replace(/\t/g, '  '))
  if (!showLineNumbers.value) {
    return `<pre style="margin:0;color:#cdd6f4;font-family:${FONT};font-size:13px;line-height:1.55;white-space:pre;">${esc}</pre>`
  }
  const lines = props.code.replace(/\t/g, '  ').split('\n')
  const nums = lines.map((_l, i) => i + 1).join('\n')
  return `<div style="display:flex;gap:14px;">`
    + `<pre style="margin:0;color:#585b70;font-family:${FONT};font-size:13px;line-height:1.55;white-space:pre;text-align:right;user-select:none;">${escapeHtml(nums)}</pre>`
    + `<pre style="margin:0;color:#cdd6f4;font-family:${FONT};font-size:13px;line-height:1.55;white-space:pre;">${esc}</pre>`
    + `</div>`
})

const chromeHtml = computed(() => {
  if (!showChrome.value) {
    return ''
  }
  const dot = (c: string) => `<span style="width:11px;height:11px;border-radius:50%;background:${c};display:inline-block;margin-right:7px;"></span>`
  return `<div style="height:34px;display:flex;align-items:center;padding:0 14px;background:#181825;">`
    + dot('#ff5f56') + dot('#ffbd2e') + dot('#27c93f')
    + `<span style="margin-left:10px;color:#9399b2;font-family:${FONT};font-size:12px;">${escapeHtml(props.fileName || props.language)}</span>`
    + `</div>`
})

const cardHtml = computed(() =>
  `<div style="display:inline-block;background:${THEMES[themeIdx.value].bg};padding:${padding.value}px;box-sizing:border-box;">`
  + `<div style="background:#1e1e2e;border-radius:10px;overflow:hidden;box-shadow:0 18px 40px rgba(0,0,0,.35);">`
  + chromeHtml.value
  + `<div style="padding:16px 20px;">${codeInner.value}</div>`
  + `</div></div>`)

// 用 SVG foreignObject 光栅化为 PNG（2x 清晰度）
const renderCanvas = async (): Promise<HTMLCanvasElement> => {
  const card = cardRef.value?.firstElementChild as HTMLElement
  if (!card) {
    throw new Error('no card')
  }
  const rect = card.getBoundingClientRect()
  const w = Math.ceil(rect.width)
  const h = Math.ceil(rect.height)
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}">`
    + `<foreignObject width="100%" height="100%"><div xmlns="http://www.w3.org/1999/xhtml">${cardHtml.value}</div></foreignObject></svg>`
  const url = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(svg)
  const dpr = 2
  return await new Promise((resolve, reject) => {
    const img = new Image()
    img.onload = () => {
      const canvas = document.createElement('canvas')
      canvas.width = w * dpr
      canvas.height = h * dpr
      const ctx = canvas.getContext('2d')!
      ctx.scale(dpr, dpr)
      ctx.drawImage(img, 0, 0)
      resolve(canvas)
    }
    img.onerror = () => reject(new Error('render failed'))
    img.src = url
  })
}

const baseName = () => (props.fileName || 'code').replace(/\.[^.]+$/, '')

const downloadImage = async () => {
  busy.value = true
  try {
    const canvas = await renderCanvas()
    canvas.toBlob((blob) => {
      if (!blob) {
        toast.error(t('codeImage.failed'))
        return
      }
      const a = document.createElement('a')
      a.href = URL.createObjectURL(blob)
      a.download = `${baseName()}.png`
      a.click()
      URL.revokeObjectURL(a.href)
      toast.success(t('codeImage.saved'))
    }, 'image/png')
  }
  catch (e: any) {
    toast.error(t('codeImage.failed') + ': ' + (e?.message || e))
  }
  finally {
    busy.value = false
  }
}

const copyImage = async () => {
  busy.value = true
  try {
    const canvas = await renderCanvas()
    await new Promise<void>((resolve, reject) => {
      canvas.toBlob(async (blob) => {
        try {
          if (!blob) {
            throw new Error('no blob')
          }
          await navigator.clipboard.write([new ClipboardItem({'image/png': blob})])
          toast.success(t('codeImage.copied'))
          resolve()
        }
        catch (err) {
          reject(err)
        }
      }, 'image/png')
    })
  }
  catch (e: any) {
    toast.error(t('codeImage.copyFailed') + ': ' + (e?.message || e))
  }
  finally {
    busy.value = false
  }
}
</script>
