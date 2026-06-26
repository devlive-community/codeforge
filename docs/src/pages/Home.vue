<template>
  <!-- Hero（始终深色，营造科技感） -->
  <section class="relative isolate overflow-hidden bg-slate-950 text-white" @mousemove="onHeroMove">
    <!-- 极光光晕 -->
    <div class="pointer-events-none absolute inset-0 -z-10">
      <div class="absolute -top-40 left-1/4 h-[28rem] w-[28rem] rounded-full bg-brand-600/30 blur-[120px] animate-blob"></div>
      <div class="absolute top-10 right-1/4 h-[26rem] w-[26rem] rounded-full bg-cyan-500/20 blur-[120px] animate-blob" style="animation-delay:-5s"></div>
      <div class="absolute bottom-[-10rem] left-1/3 h-[24rem] w-[24rem] rounded-full bg-fuchsia-600/20 blur-[120px] animate-blob" style="animation-delay:-9s"></div>
    </div>
    <!-- 网格 -->
    <div class="pointer-events-none absolute inset-0 -z-10 bg-[linear-gradient(to_right,rgba(255,255,255,.05)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,.05)_1px,transparent_1px)] bg-[size:44px_44px] [mask-image:radial-gradient(ellipse_at_50%_30%,black,transparent_70%)]"></div>
    <!-- 鼠标跟随光斑 -->
    <div ref="spot" class="spotlight-bg pointer-events-none absolute inset-0 -z-10"></div>

    <div class="max-w-6xl mx-auto px-5 pt-24 pb-20 grid lg:grid-cols-2 gap-14 items-center">
      <div>
        <RouterLink v-if="latestRelease" :to="`/release/${latestRelease.version}`"
                    class="inline-flex items-center gap-2 rounded-full border border-white/15 bg-white/5 px-3.5 py-1.5 text-sm text-slate-200 backdrop-blur hover:border-brand-400/60 transition-colors">
          <span class="relative flex h-2 w-2">
            <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-brand-400 opacity-75"></span>
            <span class="relative inline-flex h-2 w-2 rounded-full bg-brand-400"></span>
          </span>
          {{ t('hero.badge', {version: latestRelease.version}) }}
          <span class="text-slate-400">→</span>
        </RouterLink>

        <h1 class="mt-7 text-[3rem] leading-[1.05] sm:text-[4.25rem] font-extrabold tracking-tight">
          <span class="text-white">{{ t('hero.title1') }}</span><br/>
          <span class="animate-gradient bg-gradient-to-r from-brand-300 via-cyan-300 via-violet-300 to-brand-300 bg-clip-text text-transparent drop-shadow-[0_2px_20px_rgba(129,140,248,.35)]">{{ t('hero.title2') }}</span>
        </h1>
        <p class="mt-6 max-w-lg text-lg text-slate-300/90 leading-relaxed">{{ t('hero.subtitle') }}</p>

        <div class="mt-9 flex flex-wrap items-center gap-3">
          <RouterLink to="/download"
                      class="shimmer px-6 py-3 rounded-xl bg-gradient-to-r from-brand-500 to-violet-600 text-white font-medium shadow-glow hover:shadow-[0_24px_70px_-18px_rgba(99,102,241,.6)] transition-shadow">
            {{ t('hero.download') }}
          </RouterLink>
          <a href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener"
             class="px-6 py-3 rounded-xl border border-white/15 bg-white/5 text-white font-medium backdrop-blur hover:bg-white/10 transition-colors">
            {{ t('hero.github') }}
          </a>
        </div>

        <!-- 数据指标 -->
        <div class="mt-12 grid grid-cols-4 gap-4 max-w-lg">
          <div v-for="s in stats" :key="s.label">
            <div class="text-2xl sm:text-3xl font-extrabold bg-gradient-to-br from-white to-slate-400 bg-clip-text text-transparent">{{ s.value }}</div>
            <div class="mt-1 text-xs text-slate-400">{{ s.label }}</div>
          </div>
        </div>
      </div>

      <div class="lg:pl-4">
        <CodeWindow/>
      </div>
    </div>

    <!-- 语言滚动条 -->
    <div class="relative border-t border-white/10 bg-white/[.02] py-4 overflow-hidden [mask-image:linear-gradient(to_right,transparent,black_10%,black_90%,transparent)]">
      <div class="flex w-max animate-marquee gap-12 px-6 text-slate-400 font-mono text-sm">
        <span v-for="(lang, idx) in marquee" :key="idx" class="whitespace-nowrap">{{ lang }}</span>
      </div>
    </div>
  </section>

  <!-- 特性 -->
  <section class="max-w-6xl mx-auto px-5 py-20">
    <div class="max-w-2xl mb-12">
      <p class="text-sm font-semibold tracking-wide text-brand-600 dark:text-brand-400">{{ t('features.tag') }}</p>
      <h2 class="mt-2 text-3xl sm:text-4xl font-bold tracking-tight text-slate-900 dark:text-white">{{ t('features.title') }}</h2>
    </div>
    <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
      <div v-for="f in features" :key="f.key"
           class="spotlight-card group p-6 rounded-2xl border border-slate-200 dark:border-white/10 bg-white dark:bg-white/[.03] hover:-translate-y-1 transition-all duration-300"
           @mousemove="onCardMove">
        <div class="relative z-10">
          <div class="w-11 h-11 rounded-xl bg-gradient-to-br from-brand-500/15 to-violet-500/15 text-brand-600 dark:text-brand-300 flex items-center justify-center ring-1 ring-brand-500/20" v-html="f.icon"></div>
          <h3 class="mt-4 font-semibold text-lg text-slate-900 dark:text-white">{{ t(`features.items.${f.key}.title`) }}</h3>
          <p class="mt-2 text-slate-600 dark:text-slate-400 leading-relaxed">{{ t(`features.items.${f.key}.desc`) }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import CodeWindow from '../components/CodeWindow.vue'
import {latestRelease} from '../content/releases'

const {t} = useI18n()

const langArr = ['Python', 'Rust', 'Go', 'JavaScript', 'TypeScript', 'Java', 'C', 'C++', 'Ruby', 'PHP', 'Kotlin', 'Swift']
const marquee = [...langArr, ...langArr]

const stats = computed(() => [
  {value: t('stats.languages'), label: t('stats.languagesLabel')},
  {value: t('stats.workbenches'), label: t('stats.workbenchesLabel')},
  {value: t('stats.platforms'), label: t('stats.platformsLabel')},
  {value: t('stats.open'), label: t('stats.openLabel')}
])

// 鼠标跟随光效（仅客户端事件触发）
const spot = ref<HTMLElement | null>(null)
const onHeroMove = (e: MouseEvent) => {
  const el = e.currentTarget as HTMLElement
  const r = el.getBoundingClientRect()
  spot.value?.style.setProperty('--mx', `${e.clientX - r.left}px`)
  spot.value?.style.setProperty('--my', `${e.clientY - r.top}px`)
}
const onCardMove = (e: MouseEvent) => {
  const el = e.currentTarget as HTMLElement
  const r = el.getBoundingClientRect()
  el.style.setProperty('--mx', `${e.clientX - r.left}px`)
  el.style.setProperty('--my', `${e.clientY - r.top}px`)
}

const i = (path: string) =>
  `<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">${path}</svg>`

const features = [
  {key: 'run', icon: i('<path d="M13 2 4.5 13.5H11l-1 8.5 8.5-11.5H12z"/>')},
  {key: 'edit', icon: i('<path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/>')},
  {key: 'debug', icon: i('<rect x="3" y="11" width="18" height="10" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>')},
  {key: 'db', icon: i('<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14a9 3 0 0 0 18 0V5"/><path d="M3 12a9 3 0 0 0 18 0"/>')},
  {key: 'git', icon: i('<circle cx="6" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="8" r="3"/><path d="M6 9v6"/><path d="M18 11a6 6 0 0 1-6 6H9"/>')},
  {key: 'ai', icon: i('<path d="M12 3v4"/><path d="M12 17v4"/><path d="m6 6 1.5 1.5"/><path d="M16.5 16.5 18 18"/><circle cx="12" cy="12" r="4"/>')}
]
</script>
