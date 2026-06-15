import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import ToastPlugin from './plugins/toast'
import { loadKvStore } from './composables/useKvStore'
import { i18n, loadSavedLocale } from './i18n'

// 先从数据库载入全部键值（替代 localStorage），再挂载应用，保证同步读取可用
loadKvStore().finally(() => {
    loadSavedLocale() // KV 载入后恢复界面语言
    createApp(App)
        .use(ToastPlugin)
        .use(i18n)
        .mount('#app')
})
