// 按文件记忆运行输入（参数/stdin/环境变量）：切换文件时保存旧、载入新，编辑时防抖保存。
import {watch, type Ref} from 'vue'
import {debounce} from 'lodash-es'
import {kvGetJSON, kvSetJSON} from './useKvStore'

const RUN_CONFIGS_KEY = 'run-configs'
type RunConfig = { args: string, stdin: string, env: string }

export function useRunConfig(
  currentFilePath: Ref<string | null>,
  runArgs: Ref<string>,
  runStdin: Ref<string>,
  runEnv: Ref<string>
) {
  const loadRunConfigs = (): Record<string, RunConfig> =>
    kvGetJSON<Record<string, RunConfig>>(RUN_CONFIGS_KEY, {})

  // 把当前输入写入指定文件的配置（全空则删除该条）
  const saveRunConfig = (path: string) => {
    const map = loadRunConfigs()
    if (!runArgs.value && !runStdin.value && !runEnv.value) {
      delete map[path]
    }
    else {
      map[path] = {args: runArgs.value, stdin: runStdin.value, env: runEnv.value}
    }
    kvSetJSON(RUN_CONFIGS_KEY, map)
  }

  // 载入指定文件的配置（无则清空）
  const loadRunConfig = (path: string | null) => {
    const cfg = path ? loadRunConfigs()[path] : null
    runArgs.value = cfg?.args || ''
    runStdin.value = cfg?.stdin || ''
    runEnv.value = cfg?.env || ''
  }

  // 切换文件时：保存旧文件输入、载入新文件输入
  watch(currentFilePath, (np, op) => {
    if (op) {
      saveRunConfig(op)
    }
    loadRunConfig(np)
  })

  // 编辑输入时防抖保存到当前文件
  const persistRunConfig = debounce(() => {
    if (currentFilePath.value) {
      saveRunConfig(currentFilePath.value)
    }
  }, 400)
  watch([runArgs, runStdin, runEnv], () => persistRunConfig())
}
