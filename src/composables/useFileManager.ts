import {computed, type Ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {open as openFileDialog, save as saveFileDialog} from '@tauri-apps/plugin-dialog'

interface FileManagerOptions
{
    // 编辑器内容（双向绑定的 ref）
    code: Ref<string>
    toast: any
    // 另存为时的建议文件名（通常根据当前语言扩展名生成）
    getDefaultFileName: () => string
    // 当前关联的本地文件路径（由上层注入，便于与多标签共享）
    currentFilePath: Ref<string | null>
    // 最近一次保存/打开时的内容，用于判断是否有未保存改动（由上层注入）
    savedContent: Ref<string | null>
    // 选定文件、即将载入前回调（如新建标签页）
    onBeforeLoad?: () => void
    // 打开文件后回调（内容已写入编辑器），用于按扩展名切换语言等
    onOpened?: (filePath: string, content: string) => void
    // 打开文件大小上限(MB)，传给后端校验
    getMaxFileSizeMb?: () => number | undefined
}

/**
 * 编辑器本地文件管理：打开、保存、另存为。
 * 文件状态 ref 由上层注入，以便与多标签工作区共享。
 */
export function useFileManager(options: FileManagerOptions)
{
    const {code, toast, getDefaultFileName, currentFilePath, savedContent, onBeforeLoad, onOpened, getMaxFileSizeMb} = options

    const currentFileName = computed(() => {
        if (!currentFilePath.value) {
            return ''
        }
        return currentFilePath.value.split(/[\\/]/).pop() || ''
    })

    // 是否有未保存的改动
    const isDirty = computed(() => currentFilePath.value !== null && code.value !== savedContent.value)

    // 载入指定路径的文件内容到编辑器
    const loadPath = async (filePath: string) => {
        try {
            onBeforeLoad?.()

            const content = await invoke<string>('read_file_text', {
                path: filePath,
                maxSizeMb: getMaxFileSizeMb?.()
            })
            code.value = content
            currentFilePath.value = filePath
            savedContent.value = content
            onOpened?.(filePath, content)
            toast.success(`已打开 ${currentFileName.value}`)
        }
        catch (error) {
            toast.error('打开文件失败: ' + error)
        }
    }

    // 仅弹出文件选择对话框，返回选中路径（不载入），供上层按大小决定打开方式
    const pickFile = async (): Promise<string | null> => {
        const selected = await openFileDialog({multiple: false, directory: false})
        return selected && typeof selected === 'string' ? selected : null
    }

    const openFile = async () => {
        const selected = await pickFile()
        if (!selected) {
            return
        }
        await loadPath(selected)
    }

    // 打开指定路径（用于文件树点击等）
    const openPath = async (filePath: string) => {
        await loadPath(filePath)
    }

    const writeToPath = async (path: string) => {
        await invoke('write_file_text', {path, content: code.value})
        currentFilePath.value = path
        savedContent.value = code.value
    }

    const saveFileAs = async () => {
        try {
            const path = await saveFileDialog({defaultPath: getDefaultFileName()})
            if (!path) {
                return
            }
            await writeToPath(path)
            toast.success(`已保存到 ${currentFileName.value}`)
        }
        catch (error) {
            toast.error('保存文件失败: ' + error)
        }
    }

    const saveFile = async () => {
        // 尚未关联文件时走"另存为"
        if (!currentFilePath.value) {
            await saveFileAs()
            return
        }
        try {
            await writeToPath(currentFilePath.value)
            toast.success(`已保存 ${currentFileName.value}`)
        }
        catch (error) {
            toast.error('保存文件失败: ' + error)
        }
    }

    // 解除文件关联（如切换语言、加载示例后，编辑内容已不再对应原文件）
    const resetFile = () => {
        currentFilePath.value = null
        savedContent.value = null
    }

    return {
        currentFileName,
        isDirty,
        pickFile,
        openFile,
        openPath,
        saveFile,
        saveFileAs,
        resetFile
    }
}
