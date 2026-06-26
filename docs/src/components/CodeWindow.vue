<template>
  <div class="glow-border rounded-2xl animate-float">
    <div class="rounded-2xl border border-white/10 bg-slate-900/80 backdrop-blur-xl shadow-2xl overflow-hidden">
      <!-- 标题栏 -->
      <div class="flex items-center gap-2 px-4 h-10 border-b border-white/10 bg-white/5">
        <span class="w-3 h-3 rounded-full bg-red-400/90"></span>
        <span class="w-3 h-3 rounded-full bg-amber-400/90"></span>
        <span class="w-3 h-3 rounded-full bg-green-400/90"></span>
        <span class="ml-3 text-xs text-slate-400 font-mono">main.py</span>
        <span class="ml-auto text-[10px] uppercase tracking-wider text-brand-300/80">running</span>
      </div>
      <!-- 代码区 -->
      <div class="flex font-mono text-[13px] leading-6">
        <div class="select-none px-3 py-4 text-right text-slate-600 bg-white/5">
          <div v-for="n in lines.length" :key="n">{{ n }}</div>
        </div>
        <pre class="px-4 py-4 overflow-x-auto text-slate-200"><code><template v-for="(line, i) in lines" :key="i"><span v-html="line"></span>
</template></code></pre>
      </div>
      <!-- 输出区 -->
      <div class="border-t border-white/10 px-4 py-3 bg-black/30 font-mono text-[13px]">
        <span class="text-slate-500">$</span> <span class="text-slate-400">run main.py</span>
        <div class="mt-1 text-emerald-400">fib(10) = 55</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 手写 token 着色（暗色），避免引入运行时高亮库
const k = (s: string) => `<span class="text-violet-400">${s}</span>`
const f = (s: string) => `<span class="text-cyan-400">${s}</span>`
const n = (s: string) => `<span class="text-amber-300">${s}</span>`
const c = (s: string) => `<span class="text-slate-500">${s}</span>`

const lines = [
  c('# 计算斐波那契数列'),
  `${k('def')} ${f('fib')}(n):`,
  `    ${k('if')} n &lt; ${n('2')}:`,
  `        ${k('return')} n`,
  `    ${k('return')} ${f('fib')}(n - ${n('1')}) + ${f('fib')}(n - ${n('2')})`,
  '',
  `${f('print')}(${f('fib')}(${n('10')}))`
]
</script>
