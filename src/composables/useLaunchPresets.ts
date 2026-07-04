// 运行预设（launch presets）：把一组运行输入（参数/stdin/环境变量）存为命名条目，便于切换。
import {ref} from 'vue'
import {kvGetJSON, kvSetJSON} from './useKvStore'

export interface LaunchPreset {
  name: string
  args: string
  stdin: string
  env: string
}

const KEY = 'launch-presets'

export function useLaunchPresets() {
  const presets = ref<LaunchPreset[]>(kvGetJSON<LaunchPreset[]>(KEY, []))
  const persist = () => kvSetJSON(KEY, presets.value)

  const save = (name: string, args: string, stdin: string, env: string) => {
    const n = name.trim()
    if (!n) {
      return
    }
    const entry: LaunchPreset = {name: n, args, stdin, env}
    const idx = presets.value.findIndex(p => p.name === n)
    if (idx >= 0) {
      presets.value = presets.value.map((p, i) => (i === idx ? entry : p))
    }
    else {
      presets.value = [...presets.value, entry]
    }
    persist()
  }

  const remove = (name: string) => {
    presets.value = presets.value.filter(p => p.name !== name)
    persist()
  }

  return {presets, save, remove}
}
