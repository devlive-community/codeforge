// 技术博客：扫描 blog/*.md，自动生成博客列表与路由组件。

export interface BlogMeta {
  slug: string
  title: string
  date?: string
  author?: string
  description?: string
  tags?: string[]
}

const slugFromPath = (p: string): string => (p.match(/([^/]+)\.md$/)?.[1]) ?? p

// 按日期降序排序
const compareDateDesc = (a: string | undefined, b: string | undefined): number => {
  if (!a && !b) return 0
  if (!a) return 1
  if (!b) return -1
  return b.localeCompare(a)
}

// 导入所有 markdown 文件
const modules = import.meta.glob<any>('./blog/*.md', { eager: true })

const blogsData: BlogMeta[] = Object.entries(modules).map(([path, module]) => {
  const slug = slugFromPath(path)
  // 日期格式为 ISO 8601 字符串，提取 YYYY-MM-DD 部分
  const date = module.date
  const dateStr = typeof date === 'string' ? date.split('T')[0] : date
  return {
    slug,
    title: module.title || slug,
    date: dateStr,
    author: module.author,
    description: module.description,
    tags: module.tags || []
  }
})

export const blogs: BlogMeta[] = blogsData.sort((a, b) => compareDateDesc(a.date, b.date))

// 路由：每篇博客一个静态路由，便于 vite-ssg 全量预渲染
const componentLoaders = import.meta.glob('./blog/*.md')
export const blogRoutes = Object.entries(componentLoaders).map(([path, loader]) => ({
  path: `/blog/${slugFromPath(path)}`,
  component: loader
}))
