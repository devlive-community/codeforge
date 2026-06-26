import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import Markdown from 'unplugin-vue-markdown/vite'
import anchor from 'markdown-it-anchor'
import {fileURLToPath, URL} from 'node:url'

// 自建文档站：Vue 单文件组件 + Markdown（发布日志）渲染为 Vue 组件
export default defineConfig({
  base: '/',
  plugins: [
    // 注意：Markdown 必须在 vue 之前，先把 .md 转成 SFC 再交给 vue 编译
    Markdown({
      exposeFrontmatter: true,
      markdownItOptions: {html: true, linkify: true},
      markdownItSetup(md) {
        md.use(anchor, {permalink: anchor.permalink.headerLink()})
      }
    }),
    vue({include: [/\.vue$/, /\.md$/]})
  ],
  resolve: {
    alias: {'@': fileURLToPath(new URL('./src', import.meta.url))}
  },
  // vite-ssg：把所有路由静态预渲染为 HTML
  ssgOptions: {
    formatting: 'minify'
  }
})
