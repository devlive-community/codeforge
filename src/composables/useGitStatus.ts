// 文件树 Git 徽标 + 当前分支：聚合主根与各额外挂载根的状态。
import {ref, type Ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'

interface GitStatusResult {
  is_repo: boolean
  branch: string
  files: { path: string, index: string, worktree: string }[]
}

export function useGitStatus(
  rootDir: Ref<string | null>,
  extraRoots: Ref<string[]>,
  onRefreshed?: () => void
) {
  // 绝对路径 → 状态字母（M/A/D/U）
  const gitStatus = ref<Record<string, string>>({})
  const gitRepo = ref(false)
  const gitBranch = ref('')

  // 计算单个根的 Git 状态（绝对路径作 key，便于多根合并到同一张表）
  const gitStatusFor = async (root: string): Promise<{ isRepo: boolean, branch: string, map: Record<string, string> }> => {
    try {
      const s = await invoke<GitStatusResult>('git_status', {root})
      const map: Record<string, string> = {}
      if (s.is_repo) {
        for (const f of s.files) {
          const code = f.index === '?' ? 'U' : (f.worktree.trim() || f.index.trim() || 'M')
          map[`${root}/${f.path}`] = code
        }
      }
      return {isRepo: s.is_repo, branch: s.branch || '', map}
    }
    catch {
      return {isRepo: false, branch: '', map: {}}
    }
  }

  const refreshGitStatus = async () => {
    if (!rootDir.value) {
      gitStatus.value = {}
      gitRepo.value = false
      gitBranch.value = ''
      return
    }
    const primary = await gitStatusFor(rootDir.value)
    gitRepo.value = primary.isRepo
    gitBranch.value = primary.branch
    const map: Record<string, string> = {...primary.map}
    // 额外挂载的根各自可为独立仓库，合并它们的状态徽标
    if (extraRoots.value.length) {
      const extra = await Promise.all(extraRoots.value.map(er => gitStatusFor(er)))
      for (const e of extra) Object.assign(map, e.map)
    }
    gitStatus.value = map
    // HEAD 可能因提交/切换分支变化，回调刷新编辑器行内差异基线等
    onRefreshed?.()
  }

  return {gitStatus, gitRepo, gitBranch, refreshGitStatus}
}
