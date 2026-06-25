<template>
  <div class="inline-block">
    <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
            :title="t('er.title')" @click="open">
      <Network class="w-3.5 h-3.5"/>
    </button>

    <Teleport to="body">
      <div v-if="visible" class="fixed inset-0 z-50 flex items-start justify-center pt-10 px-6 pb-6" @click="visible = false">
        <div class="w-full max-w-[1100px] h-[80vh] bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
          <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
            <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
              <Network class="w-4 h-4 text-gray-400"/>
              <span>{{ t('er.title') }} · {{ activeLabel() }}</span>
              <span v-if="pickedDb" class="text-[11px] text-gray-400">/ {{ pickedDb }}</span>
              <button v-if="pickedDb" class="text-[11px] text-blue-500 hover:underline cursor-pointer" @click="backToPick">{{ t('er.switchDb') }}</button>
              <span v-if="!loading && !picking" class="text-[11px] text-gray-400">{{ t('er.summary', { tables: tables.length, fks: fks.length }) }}</span>
            </div>
            <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="visible = false">
              <X class="w-4 h-4"/>
            </button>
          </div>

          <div class="flex-1 relative overflow-auto bg-gray-50 dark:bg-gray-950">
            <div v-if="loading" class="absolute inset-0 flex items-center justify-center text-sm text-gray-400">{{ t('er.loading') }}</div>
            <div v-else-if="error" class="absolute inset-0 flex items-center justify-center text-sm text-red-500 px-6 text-center">{{ error }}</div>
            <div v-else-if="picking" class="absolute inset-0 overflow-auto p-4">
              <div class="text-xs text-gray-500 dark:text-gray-400 mb-2">{{ t('er.pickDb') }}</div>
              <div v-if="dbList.length" class="flex flex-wrap gap-2">
                <button v-for="d in dbList" :key="d" class="px-3 py-1.5 text-xs rounded border border-gray-300 dark:border-gray-600 hover:border-blue-400 hover:text-blue-500 cursor-pointer" @click="selectDb(d)">{{ d }}</button>
              </div>
              <div v-else class="text-xs text-gray-400">{{ t('er.empty') }}</div>
            </div>
            <div v-else-if="!tables.length" class="absolute inset-0 flex items-center justify-center text-sm text-gray-400">{{ t('er.empty') }}</div>
            <div v-else class="relative" :style="{ width: canvas.w + 'px', height: canvas.h + 'px' }">
              <!-- 外键连线 -->
              <svg class="absolute inset-0 pointer-events-none" :width="canvas.w" :height="canvas.h">
                <path v-for="(e, i) in edges" :key="i" :d="e.d" fill="none" stroke="#60a5fa" stroke-width="1.5"/>
              </svg>
              <!-- 表卡片 -->
              <div v-for="tbl in tables" :key="tbl.name"
                   class="absolute select-none rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-sm"
                   :style="{ left: posOf(tbl.name).x + 'px', top: posOf(tbl.name).y + 'px', width: CARD_W + 'px' }">
                <div class="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 rounded-t-md cursor-move truncate"
                     @mousedown="startDrag(tbl.name, $event)">{{ tbl.name }}</div>
                <div class="py-0.5">
                  <div v-for="col in tbl.columns" :key="col.name" class="flex items-center gap-1 px-2 text-[11px] font-mono leading-5">
                    <span class="flex-shrink-0 w-2 text-amber-500">{{ fkCols.has(tbl.name + '.' + col.name) ? '◆' : '' }}</span>
                    <span class="text-gray-800 dark:text-gray-100 truncate">{{ col.name }}</span>
                    <span class="ml-auto text-gray-400 truncate">{{ col.type }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {computed, reactive, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Network, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useDbConnections} from '../composables/useDbConnections'
import {columnsSql, fksSql} from '../utils/dbSchema'

interface Col { name: string; type: string }
interface Tbl { name: string; columns: Col[] }
interface Fk { tbl: string; col: string; refTbl: string; refCol: string }

const {t} = useI18n()
const {resolveActiveSource, activeLabel} = useDbConnections()

const CARD_W = 200
const HEADER_H = 26
const ROW_H = 20
const GAP_X = 80
const GAP_Y = 50

const visible = ref(false)
const loading = ref(false)
const error = ref('')
const tables = ref<Tbl[]>([])
const fks = ref<Fk[]>([])
// MySQL 未选库时的内联数据库选择
const picking = ref(false)
const dbList = ref<string[]>([])
const pickedDb = ref('')
const pos = reactive<Record<string, {x: number; y: number}>>({})

const cardH = (tbl: Tbl) => HEADER_H + tbl.columns.length * ROW_H + 4
const posOf = (name: string) => pos[name] || {x: 0, y: 0}

// 外键源列集合（用于打 ◆ 标记）
const fkCols = computed(() => new Set(fks.value.map(f => `${f.tbl}.${f.col}`)))

const canvas = computed(() => {
  let w = 400
  let h = 300
  for (const tbl of tables.value) {
    const p = posOf(tbl.name)
    w = Math.max(w, p.x + CARD_W + 40)
    h = Math.max(h, p.y + cardH(tbl) + 40)
  }
  return {w, h}
})

// 外键连线：源卡右缘中点 → 目标卡左缘中点（贝塞尔）
const edges = computed(() => {
  const byName = new Map(tables.value.map(t => [t.name, t]))
  const out: {d: string}[] = []
  for (const f of fks.value) {
    const s = byName.get(f.tbl)
    const tt = byName.get(f.refTbl)
    if (!s || !tt) {
      continue
    }
    const sp = posOf(f.tbl)
    const tp = posOf(f.refTbl)
    const x1 = sp.x + CARD_W
    const y1 = sp.y + cardH(s) / 2
    const x2 = tp.x
    const y2 = tp.y + cardH(tt) / 2
    const mx = (x1 + x2) / 2
    out.push({d: `M ${x1} ${y1} C ${mx} ${y1}, ${mx} ${y2}, ${x2} ${y2}`})
  }
  return out
})

// 自动网格布局
const layout = () => {
  const perRow = Math.max(1, Math.floor(1000 / (CARD_W + GAP_X)))
  let x = 20
  let y = 20
  let rowMaxH = 0
  let col = 0
  for (const tbl of tables.value) {
    pos[tbl.name] = {x, y}
    rowMaxH = Math.max(rowMaxH, cardH(tbl))
    col++
    if (col >= perRow) {
      col = 0
      x = 20
      y += rowMaxH + GAP_Y
      rowMaxH = 0
    }
    else {
      x += CARD_W + GAP_X
    }
  }
}

const groupTables = (rows: any[][]): Tbl[] => {
  const map = new Map<string, Tbl>()
  for (const row of rows) {
    const name = String(row[0])
    if (!map.has(name)) {
      map.set(name, {name, columns: []})
    }
    map.get(name)!.columns.push({name: String(row[1]), type: String(row[2] ?? '')})
  }
  return [...map.values()]
}

const runRows = async (sql: string): Promise<any[][]> => {
  const source = resolveActiveSource()
  const res = await invoke<any>('run_sql', {sql, source})
  if (res.error) {
    throw new Error(res.error)
  }
  return (res.result_sets || [])[0]?.rows || []
}

const load = async () => {
  loading.value = true
  error.value = ''
  try {
    const source = resolveActiveSource()
    // MySQL 未选具体数据库时，先列出数据库让用户在此选择
    if (source.kind === 'mysql' && !source.database && !pickedDb.value) {
      picking.value = true
      dbList.value = (await runRows(
        'SELECT schema_name FROM information_schema.schemata '
        + "WHERE schema_name NOT IN ('information_schema','mysql','performance_schema','sys') "
        + 'ORDER BY schema_name'
      )).map(r => String(r[0]))
      return
    }
    picking.value = false
    const db = source.kind === 'mysql' ? source.database || pickedDb.value || undefined : undefined
    tables.value = groupTables(await runRows(columnsSql(source.kind, db)))
    const fkQuery = fksSql(source.kind, db)
    if (fkQuery) {
      const rows = await runRows(fkQuery).catch(() => [] as any[][])
      fks.value = rows.map(r => ({tbl: String(r[0]), col: String(r[1]), refTbl: String(r[2]), refCol: String(r[3])}))
    }
    else {
      fks.value = []
    }
    layout()
  }
  catch (e: any) {
    error.value = String(e?.message || e)
  }
  finally {
    loading.value = false
  }
}

const open = () => {
  visible.value = true
  pickedDb.value = ''
  picking.value = false
  load()
}

// 选择数据库后加载该库的 ER
const selectDb = (name: string) => {
  pickedDb.value = name
  load()
}
// 返回数据库选择
const backToPick = () => {
  pickedDb.value = ''
  load()
}

// ===== 拖拽表卡 =====
let dragName = ''
let startX = 0
let startY = 0
let baseX = 0
let baseY = 0
const onDrag = (e: MouseEvent) => {
  if (!dragName) {
    return
  }
  pos[dragName] = {x: Math.max(0, baseX + (e.clientX - startX)), y: Math.max(0, baseY + (e.clientY - startY))}
}
const stopDrag = () => {
  dragName = ''
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup', stopDrag)
}
const startDrag = (name: string, e: MouseEvent) => {
  e.preventDefault()
  dragName = name
  startX = e.clientX
  startY = e.clientY
  baseX = posOf(name).x
  baseY = posOf(name).y
  document.addEventListener('mousemove', onDrag)
  document.addEventListener('mouseup', stopDrag)
}
</script>
