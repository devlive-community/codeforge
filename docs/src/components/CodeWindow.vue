<template>
  <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 shadow-soft overflow-hidden">
    <!-- 标题栏 -->
    <div class="flex items-center gap-2 px-4 h-10 border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-900/80">
      <span class="w-3 h-3 rounded-full bg-red-400"></span>
      <span class="w-3 h-3 rounded-full bg-amber-400"></span>
      <span class="w-3 h-3 rounded-full bg-green-400"></span>
      <span class="ml-3 text-xs text-slate-400 font-mono">main.py</span>
    </div>
    <!-- 代码区 -->
    <div class="flex font-mono text-[13px] leading-6">
      <div class="select-none px-3 py-4 text-right text-slate-300 dark:text-slate-600 bg-slate-50/60 dark:bg-slate-900/40">
        <div v-for="n in lines.length" :key="n">{{ n }}</div>
      </div>
      <pre class="px-4 py-4 overflow-x-auto"><code><template v-for="(line, i) in lines" :key="i"><span v-html="line"></span>
</template></code></pre>
    </div>
    <!-- 输出区 -->
    <div class="border-t border-slate-200 dark:border-slate-800 px-4 py-3 bg-slate-50 dark:bg-slate-900/60 font-mono text-[13px]">
      <span class="text-slate-400">$ </span><span class="text-slate-500 dark:text-slate-400">run main.py</span>
      <div class="mt-1 text-emerald-600 dark:text-emerald-400">fib(10) = 55</div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 简单的着色行（手写 token 高亮，避免引入运行时高亮库）
const k = (s: string) => `<span class="text-violet-500 dark:text-violet-400">${s}</span>`
const f = (s: string) => `<span class="text-blue-500 dark:text-blue-400">${s}</span>`
const n = (s: string) => `<span class="text-amber-600 dark:text-amber-400">${s}</span>`
const c = (s: string) => `<span class="text-slate-400">${s}</span>`
const t = (s: string) => `<span class="text-slate-700 dark:text-slate-200">${s}</span>`

const lines = [
  c('# 计算斐波那契数列'),
  `${k('def')} ${f('fib')}${t('(n):')}`,
  `    ${k('if')} ${t('n &lt;')} ${n('2')}${t(':')}`,
  `        ${k('return')} ${t('n')}`,
  `    ${k('return')} ${f('fib')}${t('(n -')} ${n('1')}${t(') +')} ${f('fib')}${t('(n -')} ${n('2')}${t(')')}`,
  '',
  `${f('print')}${t('(')}${f('fib')}${t('(')}${n('10')}${t('))')}`
]
</script>
