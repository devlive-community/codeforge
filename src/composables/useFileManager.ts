import {computed, ref, type Ref} from 'vue'
import {open as openFileDialog, save as saveFileDialog} from '@tauri-apps/plugin-dialog'
import {readTextFile, writeTextFile} from '@tauri-apps/plugin-fs'

/**
 * 编辑器本地文件管理：打开、保存、另存为
 *
 * @param code              编辑器内容（双向绑定的 ref）
 * @param toast             提示
 * @param getDefaultFileName 另存为时的建议文件名（通常根据当前语言扩展名生成）
 */
export function useFileManager(
    code: Ref<string>,
    toast: any,
    getDefaultFileName: () => string
)
{
    // 当前关联的本地文件路径（未保存到磁盘时为 null）
    const currentFilePath = ref<string | null>(null)
    // 最近一次保存/打开时的内容，用于判断是否有未保存改动
    const savedContent = ref<string | null>(null)

    const currentFileName = computed(() => {
        if (!currentFilePath.value) {
            return ''
        }
        return currentFilePath.value.split(/[\\/]/).pop() || ''
    })

    // 是否有未保存的改动
    const isDirty = computed(() => currentFilePath.value !== null && code.value !== savedContent.value)

    const openFile = async () => {
        try {
            const selected = await openFileDialog({multiple: false, directory: false})
            if (!selected || typeof selected !== 'string') {
                return
            }

            const content = await readTextFile(selected)
            code.value = content
            currentFilePath.value = selected
            savedContent.value = content
            toast.success(`已打开 ${currentFileName.value}`)
        }
        catch (error) {
            toast.error('打开文件失败: ' + error)
        }
    }

    const writeToPath = async (path: string) => {
        await writeTextFile(path, code.value)
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
        currentFilePath,
        currentFileName,
        isDirty,
        openFile,
        saveFile,
        saveFileAs,
        resetFile
    }
}
