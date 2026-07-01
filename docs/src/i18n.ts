import {createI18n} from 'vue-i18n'

export const SUPPORTED_LOCALES = ['zh', 'en'] as const
export type Locale = (typeof SUPPORTED_LOCALES)[number]

const messages = {
  zh: {
    nav: {download: '下载', blog: '博客', releases: '发布日志'},
    hero: {
      badge: 'v{version} 已发布',
      title1: '轻量级桌面',
      title2: '代码执行器',
      subtitle: '高性能、可扩展的桌面代码执行器，集编辑、运行、调试、数据库与 Git 工作台于一身，专为开发者、学生和编程爱好者打造。',
      download: '立即下载',
      github: '在 GitHub 查看',
      langsPrefix: '支持',
      langsSuffix: '等数十种语言'
    },
    features: {
      tag: '核心优势',
      title: '一个应用，覆盖完整开发流',
      items: {
        run: {title: '极速运行', desc: '一键运行任意语言，可扩展的语言支持系统，启动与执行都很迅捷。'},
        edit: {title: '智能编辑', desc: 'LSP 补全、跳转、诊断，符号大纲、面包屑、多光标与代码片段一应俱全。'},
        debug: {title: '可视化调试', desc: '基于 DAP 的断点调试，变量、调用栈、监视与悬停求值，无需切到命令行。'},
        db: {title: '数据库工作台', desc: '多数据源连接、SQL 执行与结果导出、ER 图与交互式事务，数据开发更顺手。'},
        git: {title: 'Git 工作台', desc: '暂存、提交、分支、分支图、变基、cherry-pick、子模块与工作树，全部图形化。'},
        ai: {title: 'AI 辅助', desc: '代码预测、解释、生成测试与格式化，让常见任务交给 AI 一步完成。'}
      }
    },
    download: {
      title: '下载 CodeForge',
      latestPrefix: '当前最新版本',
      latestSuffix: '，详见',
      releaseNotes: '发布说明',
      winNote: '.msi / .exe 安装包',
      macNote: 'Apple Silicon 与 Intel',
      allVersions: '查看全部历史版本与安装包'
    },
    stats: {
      languages: '数十种', languagesLabel: '支持语言',
      workbenches: '6', workbenchesLabel: '核心工作台',
      platforms: '2', platformsLabel: '跨平台支持',
      open: '100%', openLabel: '开源免费'
    },
    releaseList: {title: '发布日志', intro: '每个版本的更新内容如下，点击查看详情。'},
    footer: {download: '下载', blog: '博客', releases: '发布日志'}
  },
  en: {
    nav: {download: 'Download', blog: 'Blog', releases: 'Releases'},
    hero: {
      badge: 'v{version} released',
      title1: 'Lightweight desktop',
      title2: 'code runner',
      subtitle: 'A fast, extensible desktop code runner — editing, running, debugging, a database console and a full Git workbench in one app. Built for developers, students and coding enthusiasts.',
      download: 'Download now',
      github: 'View on GitHub',
      langsPrefix: 'Supports',
      langsSuffix: 'and dozens more'
    },
    features: {
      tag: 'Why CodeForge',
      title: 'One app for your entire workflow',
      items: {
        run: {title: 'Blazing fast', desc: 'Run any language with one click. An extensible language system keeps startup and execution snappy.'},
        edit: {title: 'Smart editing', desc: 'LSP completion, go-to, diagnostics, symbol outline, breadcrumbs, multi-cursor and snippets.'},
        debug: {title: 'Visual debugging', desc: 'DAP-based breakpoint debugging with variables, call stack, watches and hover evaluation — no terminal needed.'},
        db: {title: 'Database console', desc: 'Multiple data sources, SQL execution and export, ER diagrams and interactive transactions.'},
        git: {title: 'Git workbench', desc: 'Staging, commits, branches, branch graph, rebase, cherry-pick, submodules and worktrees — all graphical.'},
        ai: {title: 'AI assistance', desc: 'Code prediction, explanation, test generation and formatting — common tasks in a single step.'}
      }
    },
    download: {
      title: 'Download CodeForge',
      latestPrefix: 'Latest version',
      latestSuffix: '. See the',
      releaseNotes: 'release notes',
      winNote: '.msi / .exe installers',
      macNote: 'Apple Silicon & Intel',
      allVersions: 'Browse all releases and installers'
    },
    stats: {
      languages: 'Dozens', languagesLabel: 'Languages',
      workbenches: '6', workbenchesLabel: 'Workbenches',
      platforms: '2', platformsLabel: 'Platforms',
      open: '100%', openLabel: 'Open source'
    },
    releaseList: {title: 'Releases', intro: 'Update notes for each version. Click to view details.'},
    footer: {download: 'Download', blog: 'Blog', releases: 'Releases'}
  }
}

const stored = (): Locale => {
  if (typeof localStorage !== 'undefined') {
    const v = localStorage.getItem('locale')
    if (v === 'zh' || v === 'en') return v
  }
  return 'zh'
}

export const i18n = createI18n({
  legacy: false,
  locale: stored(),
  fallbackLocale: 'zh',
  messages
})

export const setLocale = (l: Locale) => {
  i18n.global.locale.value = l
  if (typeof localStorage !== 'undefined') localStorage.setItem('locale', l)
  if (typeof document !== 'undefined') document.documentElement.lang = l === 'zh' ? 'zh-CN' : 'en'
}
