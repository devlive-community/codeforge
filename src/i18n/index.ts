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
      theme: {system: '跟随系统', light: '浅色', dark: '深色'},
      runStrategy: {autoSave: '自动保存后运行', ask: '每次询问', tempCopy: '运行副本(不保存)'}
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
      theme: {system: 'Follow system', light: 'Light', dark: 'Dark'},
      runStrategy: {autoSave: 'Auto-save then run', ask: 'Ask every time', tempCopy: 'Run a copy (no save)'}
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
