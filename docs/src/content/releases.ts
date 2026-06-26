// 发布日志：扫描 release/*.md，自动生成版本列表与路由组件。
// 新发版只需往 release/ 丢一个 md 文件，无需改其它代码。

// 懒加载的 Vue 组件（每个 md 经 unplugin-vue-markdown 转成组件）
const componentLoaders = import.meta.glob('./release/*.md')

export interface ReleaseMeta {
  version: string
  title: string
}

const versionFromPath = (p: string): string => (p.match(/([^/]+)\.md$/)?.[1]) ?? p

// 形如 26.2.0 的版本号降序
const compareVersionDesc = (a: string, b: string): number => {
  const pa = a.split('.').map(Number)
  const pb = b.split('.').map(Number)
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    const d = (pb[i] || 0) - (pa[i] || 0)
    if (d !== 0) return d
  }
  return 0
}

const versions = Object.keys(componentLoaders)
  .map(versionFromPath)
  .sort(compareVersionDesc)

export const releases: ReleaseMeta[] = versions.map(version => ({version, title: `v${version}`}))

export const latestRelease = releases[0]

// 路由：每个版本一个静态路由，便于 vite-ssg 全量预渲染
export const releaseRoutes = Object.entries(componentLoaders).map(([path, loader]) => ({
  path: `/release/${versionFromPath(path)}`,
  component: loader
}))
