import type {RouteRecordRaw} from 'vue-router'
import Home from './pages/Home.vue'
import Download from './pages/Download.vue'
import ReleaseList from './pages/ReleaseList.vue'
import {releaseRoutes} from './content/releases'

export const routes: RouteRecordRaw[] = [
  {path: '/', component: Home, meta: {title: 'CodeForge — 轻量级桌面代码执行器'}},
  {path: '/download', component: Download, meta: {title: '下载 CodeForge'}},
  {path: '/release', component: ReleaseList, meta: {title: '发布日志'}},
  ...releaseRoutes.map(r => ({...r, meta: {doc: true}}))
]
