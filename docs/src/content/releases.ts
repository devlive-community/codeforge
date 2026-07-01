// 发布日志：扫描 release/*.md，自动生成版本列表与路由组件。
// 新发版只需往 release/ 丢一个 md 文件，无需改其它代码。

export interface ReleaseMeta {
  version: string
  title: string
  date?: string
  description?: string
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

// 导入所有 markdown 文件（unplugin-vue-markdown 将 frontmatter 字段直接暴露在模块根级别）
const modules = import.meta.glob<any>('./release/*.md', { eager: true })

const releasesData: ReleaseMeta[] = Object.entries(modules).map(([path, module]) => {
  const version = versionFromPath(path)
  // frontmatter 字段直接在模块根级别
  // 日期格式为 ISO 8601 字符串，提取 YYYY-MM-DD 部分
  const date = module.date
  const dateStr = typeof date === 'string' ? date.split('T')[0] : date
  return {
    version,
    title: `v${version}`,
    date: dateStr,
    description: module.description
  }
})

export const releases: ReleaseMeta[] = releasesData.sort((a, b) => compareVersionDesc(a.version, b.version))

export const latestRelease = releases[0]

// 路由：每个版本一个静态路由，便于 vite-ssg 全量预渲染
const componentLoaders = import.meta.glob('./release/*.md')
export const releaseRoutes = Object.entries(componentLoaders).map(([path, loader]) => ({
  path: `/release/${versionFromPath(path)}`,
  component: loader
}))
