import { nextTick, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-shell'
import { AppInfo } from '../types/app.ts'

export function useAboutModal(emit: any)
{
    // 状态管理
    const isVisible = ref(false)
    const platform = ref('Unknown')
    const version = ref('1.0.0')
    const buildTime = ref('2025-08-09')

    // 功能列表
    const features = [
        '多语言代码执行支持',
        '智能语法高亮系统',
        '实时执行统计分析',
        '现代化用户界面'
    ]

    // 技术栈
    const techStack = [
        { name: 'Vue 3', class: 'bg-emerald-100 dark:bg-emerald-900/50 text-emerald-800 dark:text-emerald-200', url: 'https://vuejs.org' },
        { name: 'TypeScript', class: 'bg-blue-100 dark:bg-blue-900/50 text-blue-800 dark:text-blue-200', url: 'https://www.typescriptlang.org' },
        { name: 'Tauri', class: 'bg-purple-100 dark:bg-purple-900/50 text-purple-800 dark:text-purple-200', url: 'https://tauri.app' },
        { name: 'Rust', class: 'bg-orange-100 dark:bg-orange-900/50 text-orange-800 dark:text-orange-200', url: 'https://www.rust-lang.org' },
        { name: 'Tailwind CSS', class: 'bg-cyan-100 dark:bg-cyan-900/50 text-cyan-800 dark:text-cyan-200', url: 'https://tailwindcss.com' }
    ]

    // GitHub 相关信息
    const githubInfo = {
        repository: 'https://github.com/devlive-community/codeforge',
        starUrl: 'https://github.com/devlive-community/codeforge/stargazers',
        issuesUrl: 'https://github.com/devlive-community/codeforge/issues'
    }

    // 加载应用信息
    const loadAppInfo = async () => {
        try {
            const appInfo: AppInfo = await invoke('get_app_info')
            version.value = appInfo.version
            buildTime.value = appInfo.build_time
            platform.value = `${ appInfo.platform } (${ appInfo.arch })`
        }
        catch (error) {
            console.error('Failed to get app info:', error)
            // 使用默认值
            platform.value = navigator.platform
        }
    }

    // 打开技术栈链接
    const openTechUrl = async (url: string) => {
        try {
            console.log('Opening URL:', url)
            await open(url)
        }
        catch (error) {
            console.error('Failed to open URL:', error)
        }
    }

    // 打开 GitHub 仓库
    const openGitHubRepo = async () => {
        try {
            await open(githubInfo.repository)
        }
        catch (error) {
            console.error('Failed to open GitHub repository:', error)
        }
    }

    // 给仓库点 Star
    const starRepository = async () => {
        try {
            await open(githubInfo.repository)
        }
        catch (error) {
            console.error('Failed to star repository:', error)
        }
    }

    // 报告问题
    const reportIssue = async () => {
        try {
            await open(githubInfo.issuesUrl)
        }
        catch (error) {
            console.error('Failed to open issues page:', error)
        }
    }

    // 关闭模态框
    const closeAbout = () => {
        isVisible.value = false
        setTimeout(() => {
            emit('close')
        }, 300)
    }

    // 显示模态框
    const showModal = async () => {
        await loadAppInfo()
        await nextTick()
        setTimeout(() => {
            isVisible.value = true
        }, 50)
    }

    return {
        // 状态
        isVisible,
        platform,
        version,
        buildTime,
        features,
        techStack,
        githubInfo,

        // 方法
        loadAppInfo,
        openTechUrl,
        openGitHubRepo,
        starRepository,
        reportIssue,
        closeAbout,
        showModal
    }
}
