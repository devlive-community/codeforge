import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import ToastPlugin from './plugins/toast'

createApp(App)
    .use(ToastPlugin)
    .mount('#app')
