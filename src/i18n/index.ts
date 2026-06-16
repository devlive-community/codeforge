// 国际化（vue-i18n）。默认简体中文，界面零变化；英文为增量翻译，可在通用设置切换。
// 文案按面逐步迁移，已迁移：通用设置页。
import {createI18n} from 'vue-i18n'
import {kvGet, kvSet} from '../composables/useKvStore'

export const SUPPORTED_LOCALES = [
  {value: 'zh-CN', label: '简体中文'},
  {value: 'en', label: 'English'}
]
const LOCALE_KEY = 'app-locale'

const messages = {
  'zh-CN': {
    settings: {
      title: '设置',
      uiLanguage: '界面语言',
      nav: {
        general: '通用',
        editor: '编辑器',
        shortcut: '快捷键',
        ai: 'AI',
        database: '数据库',
        language: '语言',
        lsp: '语言服务',
        network: '网络',
        cache: '缓存',
        logs: '日志'
      },
      general: {
        appearance: '外观',
        theme: '主题',
        runAndFile: '运行与文件',
        runUnsaved: '运行未保存文件时',
        selectStrategy: '选择运行策略',
        maxFileSize: '打开文件大小上限 (MB)',
        maxFileSizeHint: '超过该大小将以只读方式查看',
        github: 'GitHub 配置',
        githubToken: 'GitHub Token (可选)',
        githubInfoTitle: 'GitHub Token 说明',
        githubInfo1: '用于提高 GitHub API 请求速率限制（从 60次/小时 提升到 5000次/小时）',
        githubInfo2Pre: '在 ',
        githubInfo2Link: 'GitHub Settings',
        githubInfo2Post: ' 创建 Personal Access Token',
        githubInfo3: 'Token 不需要任何权限（public access 即可）',
        githubInfo4: '留空则使用未认证模式访问 GitHub API',
        saveGithub: '保存 GitHub 配置',
        savingGithub: '保存中...',
        clearToken: '清除 Token'
      },
      editor: {
        indentWithTab: '是否使用 Tab 缩进',
        showLineNumbers: '是否显示行号',
        showFunctionHelp: '是否显示函数帮助信息',
        spaceDotOmission: '是否显示空格省略',
        tabSize: '缩进空格数',
        font: '编辑器字体',
        fontSize: '字体大小',
        theme: '编辑器主题',
        selectTheme: '选择编辑器主题'
      },
      ai: {
        title: 'AI 助手',
        provider: '服务商',
        apiKey: 'API Key',
        apiKeyHint: '填写所选服务商的 API Key',
        model: '模型',
        baseUrl: '接口地址（可选）',
        info1: '请求经本地后端转发，避免浏览器跨域问题',
        info2: 'API Key 仅保存在本机',
        info3: '留空接口地址则使用服务商默认地址'
      },
      network: {
        title: 'CDN 镜像配置',
        enable: '启用 CDN 镜像加速',
        enabled: '已启用',
        disabled: '未启用',
        baseUrl: 'CDN 基础 URL',
        fallback: 'CDN 下载失败时自动回退到 GitHub 官方源',
        infoTitle: 'CDN 镜像说明',
        info1: 'CDN 镜像用于加速环境安装包的下载',
        info2: '启用自动回退后，CDN 下载失败会自动使用 GitHub 官方源',
        info3: '关闭自动回退后，CDN 下载失败将直接报错，不会尝试其他源',
        save: '保存配置',
        saving: '保存中...',
        reset: '重置为默认',
        test: '测试连接',
        testing: '测试中...'
      },
      shortcut: {
        resetAll: '全部重置',
        press: '按下快捷键…',
        cancel: '取消',
        edit: '修改',
        reset: '重置',
        hint: '提示：修改后关闭设置即生效。'
      },
      cache: {
        title: '缓存管理',
        desc: '管理应用缓存，包括代码执行临时文件等',
        pluginsCache: '插件执行缓存',
        totalCache: '总缓存大小',
        loading: '加载中...',
        clearPluginsTitle: '清理插件执行缓存',
        clearPluginsDesc: '清理代码执行产生的临时文件',
        clearAllTitle: '清理所有缓存',
        clearAllDesc: '清理应用所有缓存数据',
        clear: '清理',
        clearedPlugins: '插件执行缓存已清理',
        clearedAll: '所有缓存已清理'
      },
      logs: {
        title: '日志设置',
        currentDir: '当前日志目录',
        loading: '加载中...',
        selectDir: '选择新的日志目录',
        dirPlaceholder: '选择或输入日志目录路径',
        apply: '应用更改',
        openDir: '打开日志目录',
        resetDefault: '重置为默认',
        recentFiles: '最近的日志文件',
        mgmtTitle: '日志管理',
        cleanup: '清理日志',
        keepDaysPlaceholder: '选择保留天数',
        clearNow: '立即清理'
      },
      theme: {system: '跟随系统', light: '浅色', dark: '深色'},
      runStrategy: {autoSave: '自动保存后运行', ask: '每次询问', tempCopy: '运行副本(不保存)'}
    },
    header: {
      selectLanguage: '选择语言',
      run: '运行代码',
      stop: '停止执行',
      loadExample: '加载示例',
      history: '执行历史',
      openFile: '打开文件',
      saveFile: '保存文件',
      ai: 'AI 助手',
      showSidebar: '显示侧栏',
      hideSidebar: '隐藏侧栏',
      layoutHorizontal: '左右布局',
      layoutVertical: '上下布局',
      layoutEditor: '仅编辑器',
      hasUpdate: '有新版本'
    },
    status: {
      checking: '检查环境中...',
      notInstalled: '环境未安装',
      recheck: '重新检查环境',
      latency: '最新: {ms} 毫秒',
      lspIndexing: '语言服务索引中：{lang}',
      lspReady: '语言服务已就绪：{lang}（点击查看问题）',
      lspBadgeIndexing: 'LSP 索引中',
      chars: '字符',
      terminalTip: '终端（{key}）'
    },
    git: {
      notRepo: '非 Git 仓库',
      refresh: '刷新',
      close: '关闭',
      staged: '暂存的更改',
      unstageAll: '全部取消暂存',
      changes: '更改',
      stageAll: '全部暂存',
      clean: '没有未提交的更改',
      messagePlaceholder: '提交信息…（Cmd/Ctrl+Enter 提交）',
      aiGenTitle: '用 AI 根据改动生成提交信息',
      commit: '提交',
      commitPush: '提交并推送',
      push: '推送',
      statusFailed: '读取 Git 状态失败',
      stageFailed: '暂存失败',
      unstageFailed: '取消暂存失败',
      committed: '已提交',
      commitFailed: '提交失败',
      pushed: '已推送',
      pushFailed: '推送失败',
      needMessage: '请填写提交信息并暂存改动',
      committedPushed: '已提交并推送',
      commitPushFailed: '提交并推送失败',
      switched: '已切换到 {branch}',
      switchFailed: '切换分支失败',
      aiNeedKey: '请先在设置中配置 AI 的 API Key',
      noDiff: '没有可用于生成的改动',
      genFailed: '生成失败'
    },
    dialog: {
      gotoLinePlaceholder: '跳转到行（1 - {max}）',
      gotoLineHint: '共 {n} 行 · 回车跳转',
      quickOpenPlaceholder: '按文件名快速打开…',
      quickOpenLoading: '加载文件列表…',
      quickOpenEmpty: '无匹配文件',
      recent: '最近'
    }
  },
  en: {
    settings: {
      title: 'Settings',
      uiLanguage: 'Language',
      nav: {
        general: 'General',
        editor: 'Editor',
        shortcut: 'Shortcuts',
        ai: 'AI',
        database: 'Database',
        language: 'Languages',
        lsp: 'Language Server',
        network: 'Network',
        cache: 'Cache',
        logs: 'Logs'
      },
      general: {
        appearance: 'Appearance',
        theme: 'Theme',
        runAndFile: 'Run & Files',
        runUnsaved: 'When running an unsaved file',
        selectStrategy: 'Select run strategy',
        maxFileSize: 'Max file size to open (MB)',
        maxFileSizeHint: 'Larger files open in read-only view',
        github: 'GitHub',
        githubToken: 'GitHub Token (optional)',
        githubInfoTitle: 'About GitHub Token',
        githubInfo1: 'Raises GitHub API rate limit (from 60/hr to 5000/hr)',
        githubInfo2Pre: 'Create a Personal Access Token in ',
        githubInfo2Link: 'GitHub Settings',
        githubInfo2Post: '',
        githubInfo3: 'The token needs no scopes (public access is enough)',
        githubInfo4: 'Leave empty to access the GitHub API unauthenticated',
        saveGithub: 'Save GitHub config',
        savingGithub: 'Saving...',
        clearToken: 'Clear token'
      },
      editor: {
        indentWithTab: 'Use Tab for indentation',
        showLineNumbers: 'Show line numbers',
        showFunctionHelp: 'Show function help',
        spaceDotOmission: 'Show whitespace dots',
        tabSize: 'Indent size',
        font: 'Editor font',
        fontSize: 'Font size',
        theme: 'Editor theme',
        selectTheme: 'Select editor theme'
      },
      ai: {
        title: 'AI Assistant',
        provider: 'Provider',
        apiKey: 'API Key',
        apiKeyHint: 'Enter the API Key for the selected provider',
        model: 'Model',
        baseUrl: 'API endpoint (optional)',
        info1: 'Requests are proxied by the local backend to avoid CORS issues',
        info2: 'The API Key is stored only on this machine',
        info3: 'Leave the endpoint empty to use the provider default'
      },
      network: {
        title: 'CDN Mirror',
        enable: 'Enable CDN mirror acceleration',
        enabled: 'Enabled',
        disabled: 'Disabled',
        baseUrl: 'CDN base URL',
        fallback: 'Fall back to official GitHub source when CDN download fails',
        infoTitle: 'About CDN mirror',
        info1: 'CDN mirror speeds up downloading environment install packages',
        info2: 'With fallback on, a failed CDN download automatically uses the official GitHub source',
        info3: 'With fallback off, a failed CDN download errors out without trying other sources',
        save: 'Save',
        saving: 'Saving...',
        reset: 'Reset to default',
        test: 'Test connection',
        testing: 'Testing...'
      },
      shortcut: {
        resetAll: 'Reset all',
        press: 'Press a shortcut…',
        cancel: 'Cancel',
        edit: 'Edit',
        reset: 'Reset',
        hint: 'Tip: changes take effect after closing Settings.'
      },
      cache: {
        title: 'Cache',
        desc: 'Manage app cache, including temporary files from code execution',
        pluginsCache: 'Plugin execution cache',
        totalCache: 'Total cache size',
        loading: 'Loading...',
        clearPluginsTitle: 'Clear plugin execution cache',
        clearPluginsDesc: 'Clear temporary files produced by code execution',
        clearAllTitle: 'Clear all cache',
        clearAllDesc: 'Clear all cached data of the app',
        clear: 'Clear',
        clearedPlugins: 'Plugin execution cache cleared',
        clearedAll: 'All cache cleared'
      },
      logs: {
        title: 'Logs',
        currentDir: 'Current log directory',
        loading: 'Loading...',
        selectDir: 'Choose a new log directory',
        dirPlaceholder: 'Select or enter a log directory path',
        apply: 'Apply changes',
        openDir: 'Open log directory',
        resetDefault: 'Reset to default',
        recentFiles: 'Recent log files',
        mgmtTitle: 'Log management',
        cleanup: 'Clean logs',
        keepDaysPlaceholder: 'Days to keep',
        clearNow: 'Clean now'
      },
      theme: {system: 'Follow system', light: 'Light', dark: 'Dark'},
      runStrategy: {autoSave: 'Auto-save then run', ask: 'Ask every time', tempCopy: 'Run a copy (no save)'}
    },
    header: {
      selectLanguage: 'Select language',
      run: 'Run',
      stop: 'Stop',
      loadExample: 'Load example',
      history: 'History',
      openFile: 'Open file',
      saveFile: 'Save file',
      ai: 'AI Assistant',
      showSidebar: 'Show sidebar',
      hideSidebar: 'Hide sidebar',
      layoutHorizontal: 'Side-by-side',
      layoutVertical: 'Top-bottom',
      layoutEditor: 'Editor only',
      hasUpdate: 'Update available'
    },
    status: {
      checking: 'Checking environment...',
      notInstalled: 'Not installed',
      recheck: 'Recheck environment',
      latency: 'Last: {ms} ms',
      lspIndexing: 'Language server indexing: {lang}',
      lspReady: 'Language server ready: {lang} (click to view problems)',
      lspBadgeIndexing: 'LSP indexing',
      chars: 'chars',
      terminalTip: 'Terminal ({key})'
    },
    git: {
      notRepo: 'Not a Git repository',
      refresh: 'Refresh',
      close: 'Close',
      staged: 'Staged changes',
      unstageAll: 'Unstage all',
      changes: 'Changes',
      stageAll: 'Stage all',
      clean: 'No uncommitted changes',
      messagePlaceholder: 'Commit message… (Cmd/Ctrl+Enter to commit)',
      aiGenTitle: 'Generate commit message from changes with AI',
      commit: 'Commit',
      commitPush: 'Commit & push',
      push: 'Push',
      statusFailed: 'Failed to read Git status',
      stageFailed: 'Stage failed',
      unstageFailed: 'Unstage failed',
      committed: 'Committed',
      commitFailed: 'Commit failed',
      pushed: 'Pushed',
      pushFailed: 'Push failed',
      needMessage: 'Enter a commit message and stage changes',
      committedPushed: 'Committed & pushed',
      commitPushFailed: 'Commit & push failed',
      switched: 'Switched to {branch}',
      switchFailed: 'Failed to switch branch',
      aiNeedKey: 'Configure the AI API Key in Settings first',
      noDiff: 'No changes to generate from',
      genFailed: 'Generation failed'
    },
    dialog: {
      gotoLinePlaceholder: 'Go to line (1 - {max})',
      gotoLineHint: '{n} lines · Enter to jump',
      quickOpenPlaceholder: 'Quick open by file name…',
      quickOpenLoading: 'Loading files…',
      quickOpenEmpty: 'No matching files',
      recent: 'Recent'
    }
  }
}

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'zh-CN',
  messages
})

export const setLocale = (locale: string) => {
  i18n.global.locale.value = locale as 'zh-CN' | 'en'
  kvSet(LOCALE_KEY, locale)
}

export const getLocale = (): string => i18n.global.locale.value

// 启动时从 KV 恢复（须在 loadKvStore 之后调用）
export const loadSavedLocale = () => {
  const saved = kvGet(LOCALE_KEY)
  if (saved && SUPPORTED_LOCALES.some(s => s.value === saved)) {
    i18n.global.locale.value = saved as 'zh-CN' | 'en'
  }
}
