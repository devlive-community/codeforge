import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import ToastPlugin from './plugins/toast'
import { loadKvStore } from './composables/useKvStore'

// 先从数据库载入全部键值（替代 localStorage），再挂载应用，保证同步读取可用
loadKvStore().finally(() => {
    createApp(App)
        .use(ToastPlugin)
        .mount('#app')
})
