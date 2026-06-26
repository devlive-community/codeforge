/// <reference types="vite/client" />

declare module '*.vue' {
  import type {DefineComponent} from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

// unplugin-vue-markdown：.md 作为 Vue 组件导入，并暴露 frontmatter
declare module '*.md' {
  import type {DefineComponent} from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
  export const frontmatter: Record<string, any>
}
