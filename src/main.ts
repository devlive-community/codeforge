import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import ToastPlugin from './plugins/toast'
import { TooltipPlugin, vTooltip } from './plugins/tooltip'
import { loadKvStore } from './composables/useKvStore'
import { i18n, loadLocales } from './i18n'
import { loadDbConnections } from './composables/useDbConnections'

// 先从数据库载入全部键值（替代 localStorage），再挂载应用，保证同步读取可用
loadKvStore().finally(() => {
    loadLocales() // KV 载入后合并自定义语言包并恢复界面语言
    loadDbConnections() // 从独立表载入数据库连接（含旧 KV 数据迁移）
    const app = createApp(App)
    app.use(ToastPlugin)
        .use(i18n)
        .use(TooltipPlugin)
    app.directive('tooltip', vTooltip)
    app.mount('#app')
})
