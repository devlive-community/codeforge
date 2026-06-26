// 交互式 SQL 事务（单例）。后端按会话持有一条连接，跨多次执行在同一连接上跑。
import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'

const active = ref(false)

async function begin(source: any): Promise<void> {
  await invoke('tx_begin', {source})
  active.value = true
}

// 在事务连接上执行 SQL，返回与 run_sql 同结构的结果
async function exec(sql: string): Promise<any> {
  return await invoke('tx_exec', {sql})
}

async function finish(commit: boolean): Promise<void> {
  try {
    await invoke('tx_finish', {commit})
  }
  finally {
    active.value = false
  }
}

export function useSqlTxn() {
  return {active, begin, exec, finish}
}
