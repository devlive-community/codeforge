import {computed, ref, watch, type Ref} from 'vue'

export interface WorkspaceTab
{
    id: string
    language: string
    code: string
    filePath: string | null
    savedContent: string | null
}

interface WorkspaceDeps
{
    code: Ref<string>                       // live 编辑器内容
    currentLanguage: Ref<string>            // live 语言
    applyLanguage: (lang: string) => void   // 仅切语言，不动内容
    currentFilePath: Ref<string | null>
    savedContent: Ref<string | null>
    restoreFile: (filePath: string | null, savedContent: string | null) => void
}

let seq = 0
const genId = () => `tab-${Date.now()}-${seq++}`

/**
 * 多标签工作区：每个 tab 保存 {语言, 内容, 文件路径, 已保存内容} 快照。
 * live 编辑状态（code/currentLanguage/文件状态）实时同步进当前 tab；
 * 切换 tab 时把目标 tab 的快照还原到 live。单 tab 时行为与原先一致。
 */
export function useWorkspace(deps: WorkspaceDeps)
{
    const {code, currentLanguage, applyLanguage, currentFilePath, savedContent, restoreFile} = deps

    const tabs = ref<WorkspaceTab[]>([])
    const activeTabId = ref('')
    // 还原 tab 期间抑制同步 watch，避免把中间状态写错 tab
    let isRestoring = false

    // 最近关闭的标签快照栈，供「重新打开已关闭标签」恢复
    const closedStack = ref<WorkspaceTab[]>([])
    const CLOSED_LIMIT = 20
    const pushClosed = (t: WorkspaceTab) => {
        // 空白草稿（无文件且无内容）不值得记录
        if (t.filePath === null && (t.code ?? '') === '') {
            return
        }
        closedStack.value.push({...t})
        if (closedStack.value.length > CLOSED_LIMIT) {
            closedStack.value.shift()
        }
    }

    const activeTab = computed(() => tabs.value.find(t => t.id === activeTabId.value) || null)

    const captureToActive = () => {
        const t = tabs.value.find(t => t.id === activeTabId.value)
        if (!t) {
            return
        }
        t.code = code.value
        t.language = currentLanguage.value
        t.filePath = currentFilePath.value
        t.savedContent = savedContent.value
    }

    // 实时把编辑状态同步进当前 tab，使标签栏文件名/脏标记保持最新
    watch([code, currentLanguage, currentFilePath, savedContent], () => {
        if (isRestoring) {
            return
        }
        captureToActive()
    })

    const loadTab = (t: WorkspaceTab) => {
        isRestoring = true
        code.value = t.code
        applyLanguage(t.language)
        restoreFile(t.filePath, t.savedContent)
        isRestoring = false
    }

    // 用当前实时状态初始化第一个 tab
    const initFirstTab = () => {
        const t: WorkspaceTab = {
            id: genId(),
            language: currentLanguage.value,
            code: code.value,
            filePath: currentFilePath.value,
            savedContent: savedContent.value
        }
        tabs.value = [t]
        activeTabId.value = t.id
    }

    const switchTab = (id: string) => {
        if (id === activeTabId.value) {
            return
        }
        captureToActive()
        const t = tabs.value.find(t => t.id === id)
        if (!t) {
            return
        }
        activeTabId.value = id
        loadTab(t)
    }

    const newTab = (init: { language: string, code?: string, filePath?: string | null, savedContent?: string | null }) => {
        captureToActive()
        const t: WorkspaceTab = {
            id: genId(),
            language: init.language,
            code: init.code ?? '',
            filePath: init.filePath ?? null,
            savedContent: init.savedContent ?? null
        }
        tabs.value.push(t)
        activeTabId.value = t.id
        loadTab(t)
        return t
    }

    const closeTab = (id: string, fallback: { language: string }) => {
        const idx = tabs.value.findIndex(t => t.id === id)
        if (idx === -1) {
            return
        }
        const wasActive = id === activeTabId.value
        // 关闭前先把活动标签的实时内容落盘到快照，保证恢复的是最新内容
        if (wasActive) {
            captureToActive()
        }
        pushClosed(tabs.value[idx])
        tabs.value.splice(idx, 1)

        // 至少保留一个 tab
        if (tabs.value.length === 0) {
            const t: WorkspaceTab = {
                id: genId(),
                language: fallback.language,
                code: '',
                filePath: null,
                savedContent: null
            }
            tabs.value = [t]
            activeTabId.value = t.id
            loadTab(t)
            return
        }

        if (wasActive) {
            const next = tabs.value[Math.min(idx, tabs.value.length - 1)]
            activeTabId.value = next.id
            loadTab(next)
        }
    }

    // 关闭除 id 外的所有标签
    const closeOthers = (id: string) => {
        captureToActive()
        const keep = tabs.value.find(t => t.id === id)
        if (!keep) {
            return
        }
        for (const t of tabs.value) {
            if (t.id !== id) {
                pushClosed(t)
            }
        }
        tabs.value = [keep]
        if (activeTabId.value !== id) {
            activeTabId.value = id
            loadTab(keep)
        }
    }

    // 关闭 id 右侧的所有标签
    const closeToRight = (id: string) => {
        const idx = tabs.value.findIndex(t => t.id === id)
        if (idx === -1) {
            return
        }
        captureToActive()
        const activeRemoved = tabs.value.slice(idx + 1).some(t => t.id === activeTabId.value)
        for (const t of tabs.value.slice(idx + 1)) {
            pushClosed(t)
        }
        tabs.value = tabs.value.slice(0, idx + 1)
        if (activeRemoved) {
            activeTabId.value = id
            const keep = tabs.value[idx]
            if (keep) {
                loadTab(keep)
            }
        }
    }

    // 拖拽排序：把 fromId 移动到 toId 的位置
    const moveTab = (fromId: string, toId: string) => {
        const from = tabs.value.findIndex(t => t.id === fromId)
        const to = tabs.value.findIndex(t => t.id === toId)
        if (from === -1 || to === -1 || from === to) {
            return
        }
        const arr = [...tabs.value]
        const [moved] = arr.splice(from, 1)
        arr.splice(to, 0, moved)
        tabs.value = arr
    }

    // 路径是否等于 target 或在其目录下
    const isUnder = (p: string, target: string) =>
        p === target || (p.startsWith(target) && (p[target.length] === '/' || p[target.length] === '\\'))

    // 文件/目录被重命名后，同步相关标签的路径（含目录前缀）
    const updateTabPath = (oldPath: string, newPath: string) => {
        const remap = (p: string) => {
            if (p === oldPath) return newPath
            if (isUnder(p, oldPath)) return newPath + p.slice(oldPath.length)
            return p
        }
        if (currentFilePath.value) {
            const remapped = remap(currentFilePath.value)
            if (remapped !== currentFilePath.value) {
                currentFilePath.value = remapped
            }
        }
        for (const t of tabs.value) {
            if (t.filePath) {
                t.filePath = remap(t.filePath)
            }
        }
    }

    // 文件/目录被删除后，解除相关标签的文件关联（内容保留，变为未命名）
    const detachTabPath = (path: string) => {
        if (currentFilePath.value && isUnder(currentFilePath.value, path)) {
            currentFilePath.value = null
            savedContent.value = null
        }
        for (const t of tabs.value) {
            if (t.filePath && isUnder(t.filePath, path)) {
                t.filePath = null
                t.savedContent = null
            }
        }
    }

    // 重新打开最近关闭的标签；若该文件已在其它标签打开则直接切过去
    const reopenClosed = () => {
        const t = closedStack.value.pop()
        if (!t) {
            return null
        }
        if (t.filePath) {
            const existing = tabs.value.find(x => x.filePath === t.filePath)
            if (existing) {
                switchTab(existing.id)
                return existing
            }
        }
        return newTab({language: t.language, code: t.code, filePath: t.filePath, savedContent: t.savedContent})
    }

    // 当前 tab 是否为可复用的空白草稿（未关联文件且内容为空）
    const isActiveReusableScratch = () => {
        const t = activeTab.value
        return !!t && t.filePath === null && (t.code ?? '') === ''
    }

    return {
        tabs,
        activeTabId,
        activeTab,
        initFirstTab,
        switchTab,
        newTab,
        closeTab,
        closeOthers,
        closeToRight,
        reopenClosed,
        moveTab,
        updateTabPath,
        detachTabPath,
        isActiveReusableScratch
    }
}
