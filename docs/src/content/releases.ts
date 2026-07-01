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

// 手动维护版本元数据（从 frontmatter 提取）
const releasesData: ReleaseMeta[] = [
  {version: '26.3.0', title: 'v26.3.0', date: '2026-06-29', description: '可视化调试、AI 增强、数据库工具、多根工作区'},
  {version: '26.2.0', title: 'v26.2.0', date: '2026-06-18', description: '专业级 Git 工作台、12 种新语言、地图点击下钻'},
  {version: '26.1.0', title: 'v26.1.0', date: '2026-06-08', description: 'Git 集成、LSP 智能补全、符号导航'},
  {version: '26.0.0', title: 'v26.0.0', date: '2026-05-25', description: '全新架构、性能优化、UI 改版'},
  {version: '25.0.5', title: 'v25.0.5', date: '2025-09-15', description: 'JVM 生态和 Go 语言支持、环境管理优化'},
  {version: '25.0.4', title: 'v25.0.4', date: '2025-09-08', description: '稳定性提升和 Bug 修复'},
  {version: '25.0.3', title: 'v25.0.3', date: '2025-09-01', description: 'UI 优化和用户体验改进'},
  {version: '25.0.2', title: 'v25.0.2', date: '2025-08-25', description: '性能优化和错误修复'},
  {version: '25.0.1', title: 'v25.0.1', date: '2025-08-18', description: '首个版本的问题修复'},
  {version: '25.0.0', title: 'v25.0.0', date: '2025-08-11', description: '首个正式版本发布'}
]

export const releases: ReleaseMeta[] = releasesData.sort((a, b) => compareVersionDesc(a.version, b.version))

export const latestRelease = releases[0]

// 路由：每个版本一个静态路由，便于 vite-ssg 全量预渲染
const componentLoaders = import.meta.glob('./release/*.md')
export const releaseRoutes = Object.entries(componentLoaders).map(([path, loader]) => ({
  path: `/release/${versionFromPath(path)}`,
  component: loader
}))
