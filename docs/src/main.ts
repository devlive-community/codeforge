import {ViteSSG} from 'vite-ssg'
import App from './App.vue'
import {routes} from './router'
import {i18n} from './i18n'
import './styles/main.css'

// vite-ssg：开发用 SPA，构建时把每个路由预渲染成静态 HTML
export const createApp = ViteSSG(
  App,
  {routes, scrollBehavior: () => ({top: 0})},
  ({app, router}) => {
    app.use(i18n)
    router.afterEach((to) => {
      if (typeof document !== 'undefined' && to.meta?.title) {
        document.title = String(to.meta.title)
      }
    })
  }
)
