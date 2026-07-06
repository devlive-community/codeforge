<template>
  <div class="inline-block">
    <Tooltip :text="t('qb.title')">
      <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="openBuilder">
        <Blocks class="w-3.5 h-3.5"/>
      </button>
    </Tooltip>

    <Teleport to="body">
      <div v-if="visible" class="fixed inset-0 z-50 flex items-start justify-center pt-10 px-6 pb-6" @click="visible = false">
        <div class="w-full max-w-[980px] h-[84vh] bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
          <!-- 标题栏 -->
          <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
            <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
              <Blocks class="w-4 h-4 text-gray-400"/>
              <span>{{ t('qb.title') }} · {{ activeLabel() }}</span>
              <span class="text-[11px] text-gray-400 font-normal">{{ t('qb.dragHint') }}</span>
            </div>
            <div class="flex items-center gap-2">
              <!-- 已存查询：载入 -->
              <select v-if="savedList.length" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none cursor-pointer max-w-[160px]"
                      :value="''" @change="loadSaved">
                <option value="" disabled>{{ t('qb.loadSaved') }}</option>
                <option v-for="s in savedList" :key="s.name" :value="s.name">{{ s.name }}</option>
              </select>
              <!-- 保存当前 -->
              <div class="relative">
                <button class="inline-flex items-center gap-1 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="showSave = !showSave">
                  <Save class="w-3.5 h-3.5"/>{{ t('qb.save') }}
                </button>
                <div v-if="showSave" class="absolute right-0 mt-1 z-10 w-56 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded shadow-lg p-2 flex flex-col gap-1.5">
                  <input v-model="saveName" :placeholder="t('qb.savePlaceholder')" @keydown.enter="saveQuery"
                         class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 px-2 py-1 focus:outline-none"/>
                  <button class="text-xs px-2 py-1 rounded bg-blue-500 text-white hover:bg-blue-600 cursor-pointer disabled:opacity-40" :disabled="!saveName.trim() || !sql" @click="saveQuery">{{ t('qb.save') }}</button>
                  <div v-if="savedList.length" class="border-t border-gray-100 dark:border-gray-700 pt-1 max-h-32 overflow-auto">
                    <div v-for="s in savedList" :key="s.name" class="flex items-center justify-between text-xs px-1 py-0.5 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
                      <span class="truncate">{{ s.name }}</span>
                      <button class="text-gray-400 hover:text-red-500 cursor-pointer" @click="deleteSaved(s.name)"><Trash2 class="w-3 h-3"/></button>
                    </div>
                  </div>
                </div>
              </div>
              <button class="inline-flex items-center gap-1 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="reset">
                <RotateCcw class="w-3.5 h-3.5"/>{{ t('qb.reset') }}
              </button>
              <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="visible = false">
                <X class="w-4 h-4"/>
              </button>
            </div>
          </div>

          <div v-if="loading" class="flex-1 flex items-center justify-center text-sm text-gray-400">{{ t('qb.loading') }}</div>
          <div v-else-if="error" class="flex-1 flex items-center justify-center text-sm text-red-500 px-6 text-center">{{ error }}</div>
          <div v-else-if="tables.length === 0" class="flex-1 flex items-center justify-center text-sm text-gray-400 px-6 text-center">{{ t('qb.noTables') }}</div>

          <div v-else class="flex-1 flex min-h-0">
            <!-- 左：字段面板（按已用表分组的可拖拽列） -->
            <div class="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 flex flex-col min-h-0">
              <div class="px-3 py-1.5 text-[11px] uppercase tracking-wide text-gray-400 flex-shrink-0">{{ t('qb.columns') }}</div>
              <div class="overflow-auto flex-1 px-2 pb-2 flex flex-col gap-1">
                <template v-for="tn in usedTables" :key="tn">
                  <div class="flex items-center gap-1.5 px-1 pt-1 text-[11px] text-gray-500 dark:text-gray-400 font-medium">
                    <Table2 class="w-3 h-3 flex-shrink-0"/>
                    <span class="truncate">{{ tn }}</span>
                    <span class="text-[9px] px-1 rounded bg-gray-100 dark:bg-gray-800 text-gray-400">{{ tn === table ? 'FROM' : 'JOIN' }}</span>
                  </div>
                  <div v-for="c in colsOf(tn)" :key="tn + '.' + c.name"
                       draggable="true"
                       class="flex items-center gap-1.5 px-2 py-1 rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-xs cursor-grab active:cursor-grabbing hover:border-blue-400"
                       @dragstart="onDragStart(tn, c.name)" @dragend="onDragEnd">
                    <GripVertical class="w-3 h-3 text-gray-300 flex-shrink-0"/>
                    <span class="truncate">{{ c.name }}</span>
                    <span class="ml-auto text-[10px] text-gray-400 flex-shrink-0">{{ c.type }}</span>
                  </div>
                </template>
              </div>
            </div>

            <!-- 右：FROM/JOIN + 落区 + SQL 预览 -->
            <div class="flex-1 min-w-0 overflow-auto p-3 flex flex-col gap-3">
              <!-- FROM / JOIN -->
              <div class="rounded border border-gray-200 dark:border-gray-700">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">FROM / JOIN</div>
                <div class="px-2.5 pb-2 flex flex-col gap-1.5">
                  <div class="flex items-center gap-1.5">
                    <span class="text-[11px] text-gray-400 w-10 flex-shrink-0">FROM</span>
                    <select v-model="table" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none min-w-[160px]">
                      <option v-for="tb in tables" :key="tb.name" :value="tb.name">{{ tb.name }}</option>
                    </select>
                  </div>
                  <div v-for="(j, i) in joins" :key="i" class="flex items-center gap-1.5 flex-wrap">
                    <select v-model="j.type" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none flex-shrink-0">
                      <option v-for="jt in JOIN_TYPES" :key="jt" :value="jt">{{ jt }}</option>
                    </select>
                    <span class="text-[11px] text-gray-400">JOIN</span>
                    <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs font-mono">{{ j.table }}</span>
                    <span class="text-[11px] text-gray-400">ON</span>
                    <select v-model="j.leftT" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1 py-1 focus:outline-none">
                      <option v-for="lt in tablesBefore(i)" :key="lt" :value="lt">{{ lt }}</option>
                    </select>
                    <select v-model="j.leftC" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1 py-1 focus:outline-none">
                      <option v-for="c in colsOf(j.leftT)" :key="c.name" :value="c.name">{{ c.name }}</option>
                    </select>
                    <span class="text-xs text-gray-400">=</span>
                    <span class="text-[11px] text-gray-400 font-mono">{{ j.table }}.</span>
                    <select v-model="j.rightC" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1 py-1 focus:outline-none">
                      <option v-for="c in colsOf(j.table)" :key="c.name" :value="c.name">{{ c.name }}</option>
                    </select>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1" @click="removeJoin(i)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                  <select v-if="unusedTables.length" class="self-start text-xs rounded border border-dashed border-gray-300 dark:border-gray-600 bg-transparent text-blue-500 px-2 py-1 focus:outline-none cursor-pointer"
                          :value="''" @change="onAddJoin($event)">
                    <option value="" disabled>{{ t('qb.addJoin') }}</option>
                    <option v-for="tb in unusedTables" :key="tb.name" :value="tb.name">{{ tb.name }}</option>
                  </select>
                </div>
              </div>

              <!-- SELECT（函数 + 别名 + DISTINCT） -->
              <div class="rounded border transition-colors" :class="zoneClass('select')"
                   @dragover.prevent="dropHover = 'select'" @dragleave="dropHover = ''" @drop="onDropSelect">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500 flex items-center gap-2">
                  <span>SELECT</span>
                  <label class="inline-flex items-center gap-1 font-normal cursor-pointer">
                    <input type="checkbox" v-model="distinct" class="cursor-pointer"/> DISTINCT
                  </label>
                </div>
                <div class="px-2.5 pb-2 flex flex-col gap-1.5 min-h-[28px]">
                  <span v-if="selectItems.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.selectEmpty') }}</span>
                  <div v-if="isDragging" class="border-2 border-dashed rounded px-2 py-1 text-xs text-center pointer-events-none transition-colors"
                       :class="dropHover === 'select' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedRef?.c }) }}
                  </div>
                  <div v-for="(it, i) in selectItems" :key="i" class="flex items-center gap-1.5">
                    <select v-model="it.fn" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none flex-shrink-0">
                      <option v-for="fn in FUNCS" :key="fn" :value="fn">{{ fn || t('qb.noFn') }}</option>
                    </select>
                    <span class="px-2 py-0.5 rounded bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-300 text-xs font-mono flex-shrink-0">{{ refLabel(it) }}</span>
                    <span class="text-[11px] text-gray-400 flex-shrink-0">AS</span>
                    <input v-model="it.alias" :placeholder="t('qb.aliasPlaceholder')"
                           class="flex-1 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1 flex-shrink-0" @click="selectItems.splice(i, 1)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                </div>
              </div>

              <!-- WHERE -->
              <div class="rounded border transition-colors" :class="zoneClass('where')"
                   @dragover.prevent="dropHover = 'where'" @dragleave="dropHover = ''" @drop="onDropWhere">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">WHERE</div>
                <div class="px-2.5 pb-2 flex flex-col gap-1.5 min-h-[28px]">
                  <span v-if="wheres.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.whereEmpty') }}</span>
                  <div v-if="isDragging" class="border-2 border-dashed rounded px-2 py-1 text-xs text-center pointer-events-none transition-colors"
                       :class="dropHover === 'where' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedRef?.c }) }}
                  </div>
                  <div v-for="(w, i) in wheres" :key="i" class="flex items-center gap-1.5">
                    <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs font-mono flex-shrink-0">{{ refLabel(w) }}</span>
                    <select v-model="w.op" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none flex-shrink-0">
                      <option v-for="op in opsFor(w)" :key="op" :value="op">{{ opLabel(op) }}</option>
                    </select>
                    <template v-if="w.op !== 'IS NULL' && w.op !== 'IS NOT NULL'">
                      <template v-if="w.op === 'BETWEEN'">
                        <input v-model="w.value" :type="inputType(w)" :placeholder="t('qb.value')"
                               class="w-28 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                        <span class="text-xs text-gray-400">AND</span>
                        <input v-model="w.value2" :type="inputType(w)" :placeholder="t('qb.value')"
                               class="w-28 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                      </template>
                      <select v-else-if="colType(w) === 'boolean'" v-model="w.value"
                              class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none">
                        <option value="true">TRUE</option>
                        <option value="false">FALSE</option>
                      </select>
                      <input v-else v-model="w.value" :type="w.op === 'IN' ? 'text' : inputType(w)"
                             :placeholder="w.op === 'IN' ? t('qb.inPlaceholder') : t('qb.value')"
                             class="flex-1 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                    </template>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1 ml-auto flex-shrink-0" @click="wheres.splice(i, 1)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                </div>
              </div>

              <!-- GROUP BY -->
              <div class="rounded border transition-colors" :class="zoneClass('group')"
                   @dragover.prevent="dropHover = 'group'" @dragleave="dropHover = ''" @drop="onDropGroup">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500 flex items-center gap-2">
                  <span>GROUP BY</span>
                  <button v-if="hasAggregate" class="font-normal text-blue-500 hover:underline cursor-pointer" @click="autoGroupBy">{{ t('qb.autoGroup') }}</button>
                </div>
                <div class="px-2.5 pb-2 flex flex-wrap gap-1.5 min-h-[28px] items-center">
                  <span v-if="groupBy.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.groupEmpty') }}</span>
                  <span v-if="isDragging" class="border-2 border-dashed rounded px-2 py-0.5 text-xs pointer-events-none transition-colors"
                        :class="dropHover === 'group' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedRef?.c }) }}
                  </span>
                  <span v-for="(g, i) in groupBy" :key="refKey(g)" class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs font-mono">
                    {{ refLabel(g) }}
                    <button class="hover:text-red-500 cursor-pointer" @click="groupBy.splice(i, 1)"><X class="w-3 h-3"/></button>
                  </span>
                </div>
              </div>

              <!-- ORDER BY -->
              <div class="rounded border transition-colors" :class="zoneClass('order')"
                   @dragover.prevent="dropHover = 'order'" @dragleave="dropHover = ''" @drop="onDropOrder">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">ORDER BY</div>
                <div class="px-2.5 pb-2 flex flex-wrap gap-1.5 min-h-[28px] items-center">
                  <span v-if="orders.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.orderEmpty') }}</span>
                  <span v-if="isDragging" class="border-2 border-dashed rounded px-2 py-0.5 text-xs pointer-events-none transition-colors"
                        :class="dropHover === 'order' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedRef?.c }) }}
                  </span>
                  <span v-for="(o, i) in orders" :key="refKey(o)" class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs">
                    {{ refLabel(o) }}
                    <button class="text-blue-500 hover:underline cursor-pointer font-mono" @click="o.dir = o.dir === 'ASC' ? 'DESC' : 'ASC'">{{ o.dir }}</button>
                    <button class="hover:text-red-500 cursor-pointer" @click="orders.splice(i, 1)"><X class="w-3 h-3"/></button>
                  </span>
                </div>
              </div>

              <!-- HAVING（对聚合结果过滤，仅存在聚合时可用） -->
              <div v-if="hasAggregate" class="rounded border transition-colors" :class="zoneClass('having')"
                   @dragover.prevent="dropHover = 'having'" @dragleave="dropHover = ''" @drop="onDropHaving">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">HAVING</div>
                <div class="px-2.5 pb-2 flex flex-col gap-1.5 min-h-[28px]">
                  <span v-if="havings.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.havingEmpty') }}</span>
                  <div v-if="isDragging" class="border-2 border-dashed rounded px-2 py-1 text-xs text-center pointer-events-none transition-colors"
                       :class="dropHover === 'having' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedRef?.c }) }}
                  </div>
                  <div v-for="(h, i) in havings" :key="i" class="flex items-center gap-1.5">
                    <select v-model="h.fn" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none flex-shrink-0">
                      <option v-for="fn in AGG_FUNCS" :key="fn" :value="fn">{{ fn || t('qb.noFn') }}</option>
                    </select>
                    <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs font-mono flex-shrink-0">{{ refLabel(h) }}</span>
                    <select v-model="h.op" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none flex-shrink-0">
                      <option v-for="op in NUM_OPS" :key="op" :value="op">{{ op }}</option>
                    </select>
                    <input v-model="h.value" :placeholder="t('qb.value')"
                           class="flex-1 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1 flex-shrink-0" @click="havings.splice(i, 1)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                </div>
              </div>

              <!-- LIMIT -->
              <div class="flex items-center gap-2">
                <label class="text-[11px] font-medium text-gray-500">LIMIT</label>
                <input v-model.number="limit" type="number" min="0" class="w-24 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
              </div>

              <!-- SQL 预览 -->
              <div class="mt-auto">
                <div class="text-[11px] font-medium text-gray-500 mb-1">{{ t('qb.preview') }}</div>
                <pre class="text-xs font-mono bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded p-2 whitespace-pre-wrap break-all">{{ sql }}</pre>
              </div>
            </div>
          </div>

          <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-40" @click="copySql" :disabled="!sql">{{ t('qb.copy') }}</button>
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-40" @click="doInsert" :disabled="!sql">{{ t('qb.insert') }}</button>
            <button class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer" @click="doRun" :disabled="!sql">{{ t('qb.run') }}</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Blocks, GripVertical, RotateCcw, Save, Table2, Trash2, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import Tooltip from '../ui/Tooltip.vue'
import {useDbConnections} from '../composables/useDbConnections'
import {kvGetJSON, kvSetJSON} from '../composables/useKvStore'
import {useToast} from '../plugins/toast'
import {columnsSql, groupTables, quoteIdent, type Col, type Tbl} from '../utils/dbSchema'

const emit = defineEmits<{ preview: [sql: string]; insert: [sql: string] }>()
const {t} = useI18n()
const toast = useToast()
const {resolveActiveSource, activeLabel, activeRef} = useDbConnections()

type TypeCat = 'number' | 'string' | 'date' | 'boolean' | 'other'
interface ColRef { t: string; c: string }
interface Join { table: string; type: string; leftT: string; leftC: string; rightC: string }
interface SelItem extends ColRef { fn: string; alias: string }
interface Cond extends ColRef { op: string; value: string; value2?: string }
interface Ord extends ColRef { dir: 'ASC' | 'DESC' }
interface Having extends ColRef { fn: string; op: string; value: string }

const JOIN_TYPES = ['INNER', 'LEFT', 'RIGHT', 'FULL']
const FUNCS = ['', 'COUNT', 'SUM', 'AVG', 'MIN', 'MAX', 'COUNT DISTINCT']
const AGG_FUNCS = ['COUNT', 'SUM', 'AVG', 'MIN', 'MAX']
const NUM_OPS = ['=', '!=', '>', '>=', '<', '<=']

const OPS_BY_CAT: Record<TypeCat, string[]> = {
  number: ['=', '!=', '>', '>=', '<', '<=', 'BETWEEN', 'IN', 'IS NULL', 'IS NOT NULL'],
  string: ['contains', 'starts', 'ends', '=', '!=', 'LIKE', 'NOT LIKE', 'IN', 'IS NULL', 'IS NOT NULL'],
  date: ['=', '!=', '>', '>=', '<', '<=', 'BETWEEN', 'IS NULL', 'IS NOT NULL'],
  boolean: ['=', 'IS NULL', 'IS NOT NULL'],
  other: ['=', '!=', 'IS NULL', 'IS NOT NULL']
}

const typeCategory = (type: string): TypeCat => {
  const s = (type || '').toLowerCase()
  if (/bool|\bbit\b|tinyint\(1\)/.test(s)) {
    return 'boolean'
  }
  if (/int|dec|numeric|real|double|float|serial|money|number/.test(s)) {
    return 'number'
  }
  if (/date|time|timestamp|year/.test(s)) {
    return 'date'
  }
  if (/char|text|clob|string|uuid|enum|json/.test(s)) {
    return 'string'
  }
  return 'other'
}

const visible = ref(false)
const loading = ref(false)
const error = ref('')
const tables = ref<Tbl[]>([])

const table = ref('')
const joins = ref<Join[]>([])
const selectItems = ref<SelItem[]>([])
const distinct = ref(false)
const groupBy = ref<ColRef[]>([])
const wheres = ref<Cond[]>([])
const havings = ref<Having[]>([])
const orders = ref<Ord[]>([])
const limit = ref(100)

const hasAggregate = computed(() => selectItems.value.some(it => it.fn))

// ---- 表 / 列 基础 ----
const colsOf = (tableName: string): Col[] => tables.value.find(tb => tb.name === tableName)?.columns ?? []
const usedTables = computed(() => [table.value, ...joins.value.map(j => j.table)].filter(Boolean))
const multiTable = computed(() => usedTables.value.length > 1)
const unusedTables = computed(() => tables.value.filter(tb => !usedTables.value.includes(tb.name)))
// 第 i 个 join 的 ON 左侧可选表：其之前已引入的表
const tablesBefore = (i: number) => [table.value, ...joins.value.slice(0, i).map(j => j.table)]

const refKey = (r: ColRef) => `${r.t}.${r.c}`
const refLabel = (r: ColRef) => (multiTable.value ? `${r.t}.${r.c}` : r.c)
const validRef = (r: ColRef) => usedTables.value.includes(r.t) && colsOf(r.t).some(c => c.name === r.c)

const colType = (r: ColRef): TypeCat => typeCategory(colsOf(r.t).find(c => c.name === r.c)?.type ?? '')
const opsFor = (r: ColRef) => OPS_BY_CAT[colType(r)]
const inputType = (r: ColRef) => (colType(r) === 'date' ? 'date' : 'text')
const opLabel = (op: string) =>
  op === 'contains' ? t('qb.opContains') : op === 'starts' ? t('qb.opStarts') : op === 'ends' ? t('qb.opEnds') : op
const defaultOp = (r: ColRef) => (colType(r) === 'string' ? 'contains' : '=')

// ---- 拖拽 ----
const draggedRef = ref<ColRef | null>(null)
const dropHover = ref('')
const isDragging = ref(false)

const onDragStart = (tn: string, c: string) => {
  draggedRef.value = {t: tn, c}
  isDragging.value = true
  dropHover.value = ''
}
const onDragEnd = () => {
  isDragging.value = false
  dropHover.value = ''
  draggedRef.value = null
}
const onDropSelect = () => {
  const r = draggedRef.value
  if (r) {
    selectItems.value.push({t: r.t, c: r.c, fn: '', alias: ''})
  }
  onDragEnd()
}
const onDropWhere = () => {
  const r = draggedRef.value
  if (r) {
    wheres.value.push({t: r.t, c: r.c, op: defaultOp(r), value: '', value2: ''})
  }
  onDragEnd()
}
const onDropOrder = () => {
  const r = draggedRef.value
  if (r && !orders.value.some(o => o.t === r.t && o.c === r.c)) {
    orders.value.push({t: r.t, c: r.c, dir: 'ASC'})
  }
  onDragEnd()
}
const onDropGroup = () => {
  const r = draggedRef.value
  if (r && !groupBy.value.some(g => g.t === r.t && g.c === r.c)) {
    groupBy.value.push({t: r.t, c: r.c})
  }
  onDragEnd()
}
const onDropHaving = () => {
  const r = draggedRef.value
  if (r) {
    havings.value.push({t: r.t, c: r.c, fn: 'COUNT', op: '>', value: ''})
  }
  onDragEnd()
}

// ---- JOIN 增删 ----
const guessOn = (leftT: string, rightT: string) => {
  const l = colsOf(leftT).map(c => c.name)
  const r = colsOf(rightT).map(c => c.name)
  const leftC = l.includes('id') ? 'id' : l[0] || ''
  const rightC = r.find(c => c === `${leftT}_id` || c === leftC) ?? (r.includes('id') ? 'id' : r[0] || '')
  return {leftC, rightC}
}
const onAddJoin = (e: Event) => {
  const name = (e.target as HTMLSelectElement).value
  ;(e.target as HTMLSelectElement).value = ''
  if (!name || usedTables.value.includes(name)) {
    return
  }
  const on = guessOn(table.value, name)
  joins.value.push({table: name, type: 'INNER', leftT: table.value, leftC: on.leftC, rightC: on.rightC})
}
const pruneRefs = () => {
  selectItems.value = selectItems.value.filter(validRef)
  wheres.value = wheres.value.filter(validRef)
  groupBy.value = groupBy.value.filter(validRef)
  orders.value = orders.value.filter(validRef)
  havings.value = havings.value.filter(validRef)
}
const removeJoin = (i: number) => {
  const removed = joins.value[i].table
  joins.value.splice(i, 1)
  // 连带移除以被删表作为左表的后续 join，再清理失效引用
  joins.value = joins.value.filter(j => j.leftT !== removed)
  pruneRefs()
}

const zoneClass = (zone: string) =>
  dropHover.value === zone
    ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/10'
    : 'border-gray-200 dark:border-gray-700'

// 自动分组：把 SELECT 中未套聚合函数的列填入 GROUP BY
const autoGroupBy = () => {
  groupBy.value = selectItems.value.filter(it => !it.fn).map(it => ({t: it.t, c: it.c}))
}

// 切换主表：清空所有配置（恢复状态期间不触发）
let restoring = false
watch(table, () => {
  if (restoring) {
    return
  }
  joins.value = []
  selectItems.value = []
  distinct.value = false
  groupBy.value = []
  wheres.value = []
  havings.value = []
  orders.value = []
})

// ---- SQL 生成 ----
const kind = () => resolveActiveSource().kind
const q = (name: string) => quoteIdent(kind(), name)
const qRef = (r: ColRef) => (multiTable.value ? `${q(r.t)}.${q(r.c)}` : q(r.c))

const itemSql = (it: SelItem): string => {
  const base = it.fn === 'COUNT DISTINCT'
    ? `COUNT(DISTINCT ${qRef(it)})`
    : it.fn
      ? `${it.fn}(${qRef(it)})`
      : qRef(it)
  return it.alias.trim() ? `${base} AS ${q(it.alias.trim())}` : base
}

const strLit = (s: string) => `'${String(s ?? '').replace(/'/g, "''")}'`
const litByCat = (cat: TypeCat, v: string): string => {
  const s = String(v ?? '')
  if (cat === 'boolean') {
    return s === 'true' || s === '1' ? '1' : '0'
  }
  if (cat === 'number' && s.trim() !== '' && !isNaN(Number(s))) {
    return s
  }
  return strLit(s)
}

const condSql = (w: Cond): string => {
  const col = qRef(w)
  const cat = colType(w)
  switch (w.op) {
    case 'IS NULL':
    case 'IS NOT NULL':
      return `${col} ${w.op}`
    case 'BETWEEN':
      return (w.value.trim() && (w.value2 ?? '').trim())
        ? `${col} BETWEEN ${litByCat(cat, w.value)} AND ${litByCat(cat, w.value2 || '')}`
        : ''
    case 'IN': {
      const items = w.value.split(',').map(s => s.trim()).filter(Boolean).map(v => litByCat(cat, v))
      return items.length ? `${col} IN (${items.join(', ')})` : ''
    }
    case 'contains':
      return w.value.trim() ? `${col} LIKE ${strLit('%' + w.value + '%')}` : ''
    case 'starts':
      return w.value.trim() ? `${col} LIKE ${strLit(w.value + '%')}` : ''
    case 'ends':
      return w.value.trim() ? `${col} LIKE ${strLit('%' + w.value)}` : ''
    case 'LIKE':
    case 'NOT LIKE':
      return w.value.trim() ? `${col} ${w.op} ${strLit(w.value)}` : ''
    default:
      return w.value.trim() ? `${col} ${w.op} ${litByCat(cat, w.value)}` : ''
  }
}

// HAVING 条件（聚合 + 数值比较）
const havingSql = (h: Having): string => {
  if (!h.value.trim()) {
    return ''
  }
  const agg = `${h.fn}(${qRef(h)})`
  const v = !isNaN(Number(h.value)) ? h.value : strLit(h.value)
  return `${agg} ${h.op} ${v}`
}

const sql = computed(() => {
  if (!table.value) {
    return ''
  }
  const cols = selectItems.value.length === 0 ? '*' : selectItems.value.map(itemSql).join(', ')
  let out = `SELECT ${distinct.value ? 'DISTINCT ' : ''}${cols} FROM ${q(table.value)}`
  for (const j of joins.value) {
    out += ` ${j.type} JOIN ${q(j.table)} ON ${qRef({t: j.leftT, c: j.leftC})} = ${q(j.table)}.${q(j.rightC)}`
  }
  const conds = wheres.value.map(condSql).filter(Boolean)
  if (conds.length) {
    out += ' WHERE ' + conds.join(' AND ')
  }
  if (groupBy.value.length) {
    out += ' GROUP BY ' + groupBy.value.map(qRef).join(', ')
  }
  const havs = havings.value.map(havingSql).filter(Boolean)
  if (havs.length) {
    out += ' HAVING ' + havs.join(' AND ')
  }
  if (orders.value.length) {
    out += ' ORDER BY ' + orders.value.map(o => `${qRef(o)} ${o.dir}`).join(', ')
  }
  if (limit.value && limit.value > 0) {
    out += ` LIMIT ${limit.value}`
  }
  return out
})

// ---- 状态记忆：按数据源持久化 ----
interface QbState {
  table: string
  joins?: Join[]
  selectItems?: any[]
  distinct?: boolean
  groupBy?: any[]
  wheres?: any[]
  havings?: Having[]
  orders?: any[]
  limit: number
  selectedCols?: string[] // 旧版本字段
}
const stateKey = () => `qb-state:${activeRef.value}`
const persist = () => {
  if (restoring || !visible.value || !table.value) {
    return
  }
  kvSetJSON(stateKey(), {
    table: table.value,
    joins: joins.value,
    selectItems: selectItems.value,
    distinct: distinct.value,
    groupBy: groupBy.value,
    wheres: wheres.value,
    havings: havings.value,
    orders: orders.value,
    limit: limit.value
  })
}
watch([table, joins, selectItems, distinct, groupBy, wheres, havings, orders, limit], persist, {deep: true})

// 把任意（含旧版 {col}）引用规整为 {t,c}
const asRef = (o: any, fallbackT: string): ColRef => ({t: o?.t ?? fallbackT, c: o?.c ?? o?.col ?? o})

// 应用一份持久化状态（供自动恢复与载入已存查询复用）
const applyState = async (saved: QbState) => {
  if (!tables.value.some(tb => tb.name === saved.table)) {
    return false
  }
  restoring = true
  table.value = saved.table
  joins.value = (saved.joins || []).filter(j =>
    tables.value.some(tb => tb.name === j.table) && [saved.table, ...(saved.joins || []).map(x => x.table)].includes(j.leftT))
  const items = saved.selectItems ?? (saved.selectedCols || []).map((c: string) => ({t: saved.table, c, fn: '', alias: ''}))
  selectItems.value = items.map((o: any) => ({...asRef(o, saved.table), fn: o.fn || '', alias: o.alias || ''})).filter(validRef)
  distinct.value = !!saved.distinct
  groupBy.value = (saved.groupBy || []).map((o: any) => asRef(o, saved.table)).filter(validRef)
  wheres.value = (saved.wheres || []).map((o: any) => ({...asRef(o, saved.table), op: o.op, value: o.value ?? '', value2: o.value2 ?? ''})).filter(validRef)
  havings.value = (saved.havings || []).map((o: any) => ({...asRef(o, saved.table), fn: o.fn || 'COUNT', op: o.op || '>', value: o.value ?? ''})).filter(validRef)
  orders.value = (saved.orders || []).map((o: any) => ({...asRef(o, saved.table), dir: o.dir || 'ASC'})).filter(validRef)
  limit.value = saved.limit ?? 100
  await nextTick()
  restoring = false
  return true
}
const restoreState = () => {
  const saved = kvGetJSON<QbState | null>(stateKey(), null)
  return saved ? applyState(saved) : Promise.resolve(false)
}

// ---- 已存查询：按数据源命名存取 ----
interface SavedQuery { name: string; state: QbState }
const savedKey = () => `qb-saved:${activeRef.value}`
const savedList = ref<SavedQuery[]>([])
const saveName = ref('')
const showSave = ref(false)
const currentState = (): QbState => ({
  table: table.value,
  joins: joins.value,
  selectItems: selectItems.value,
  distinct: distinct.value,
  groupBy: groupBy.value,
  wheres: wheres.value,
  havings: havings.value,
  orders: orders.value,
  limit: limit.value
})
const saveQuery = () => {
  const name = saveName.value.trim()
  if (!name) {
    return
  }
  const idx = savedList.value.findIndex(s => s.name === name)
  const entry = {name, state: currentState()}
  if (idx >= 0) {
    savedList.value[idx] = entry
  }
  else {
    savedList.value.push(entry)
  }
  kvSetJSON(savedKey(), savedList.value)
  saveName.value = ''
  showSave.value = false
  toast.success(t('qb.savedOk', {name}))
}
const loadSaved = async (e: Event) => {
  const name = (e.target as HTMLSelectElement).value
  ;(e.target as HTMLSelectElement).value = ''
  const found = savedList.value.find(s => s.name === name)
  if (found) {
    await applyState(found.state)
    persist()
  }
}
const deleteSaved = (name: string) => {
  savedList.value = savedList.value.filter(s => s.name !== name)
  kvSetJSON(savedKey(), savedList.value)
}

const reset = () => {
  restoring = true
  joins.value = []
  selectItems.value = []
  distinct.value = false
  groupBy.value = []
  wheres.value = []
  havings.value = []
  orders.value = []
  limit.value = 100
  nextTick(() => {
    restoring = false
    persist()
  })
}

const load = async () => {
  loading.value = true
  error.value = ''
  try {
    const source = resolveActiveSource()
    const db = source.kind === 'mysql' ? source.database || undefined : undefined
    const res = await invoke<any>('run_sql', {sql: columnsSql(source.kind, db), source})
    if (res.error) {
      throw new Error(res.error)
    }
    tables.value = groupTables((res.result_sets || [])[0]?.rows || [])
    savedList.value = kvGetJSON<SavedQuery[]>(savedKey(), [])
    const restored = await restoreState()
    if (!restored && tables.value.length && !tables.value.find(tb => tb.name === table.value)) {
      table.value = tables.value[0].name
    }
  }
  catch (e: any) {
    error.value = e?.message || String(e)
    tables.value = []
  }
  finally {
    loading.value = false
  }
}

const openBuilder = () => {
  visible.value = true
  load()
}

const copySql = async () => {
  if (!sql.value) {
    return
  }
  try {
    await navigator.clipboard.writeText(sql.value)
    toast.success(t('qb.copied'))
  }
  catch { /* 忽略 */ }
}
const doInsert = () => {
  if (!sql.value) {
    return
  }
  emit('insert', sql.value)
  visible.value = false
}
const doRun = () => {
  if (!sql.value) {
    return
  }
  emit('preview', sql.value)
  visible.value = false
}
</script>
