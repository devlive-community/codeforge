// 表格数据 → 图表数据的塑形工具（与具体数据源解耦：SQL 结果、CSV 等均可复用）

export type AggKind = 'sum' | 'count' | 'avg' | 'max' | 'min'

export interface TableData {
  columns: string[]
  rows: any[][]
}

export interface ShapedSeries {
  name: string
  data: (number | null)[]
}

export interface ShapedData {
  categories: string[]
  series: ShapedSeries[]
}

export const AGG_LABELS: Record<AggKind, string> = {
  sum: '求和',
  count: '计数',
  avg: '平均',
  max: '最大',
  min: '最小'
}

/** 判断某列是否数值型（抽样前若干非空值） */
export function isNumericColumn(rows: any[][], colIndex: number): boolean {
  let seen = 0
  for (const row of rows) {
    const v = row[colIndex]
    if (v === null || v === undefined || v === '') {
      continue
    }
    seen++
    if (typeof v !== 'number' && isNaN(Number(v))) {
      return false
    }
    if (seen >= 20) {
      break
    }
  }
  return seen > 0
}

function aggregate(values: number[], kind: AggKind): number {
  if (values.length === 0) {
    return 0
  }
  switch (kind) {
    case 'sum':
      return values.reduce((a, b) => a + b, 0)
    case 'avg':
      return values.reduce((a, b) => a + b, 0) / values.length
    case 'max':
      return Math.max(...values)
    case 'min':
      return Math.min(...values)
    case 'count':
      return values.length
  }
}

const norm = (v: any): string => (v === null || v === undefined || v === '' ? '(空)' : String(v))

/**
 * 多维度透视聚合：
 * - dimensions[0]：分类轴（X）
 * - dimensions[1..]：分组维度，每个不同组合拆成一条 series
 * - metrics：一个或多个数值列；多指标时与分组组合，series 名为「组合 · 指标」
 * - agg：聚合方式；count 统计非空行数
 */
export function pivot(
  data: TableData,
  dimensions: string[],
  metrics: string[],
  agg: AggKind
): ShapedData {
  const dims = dimensions.filter(d => data.columns.includes(d))
  const mets = metrics.filter(m => data.columns.includes(m))
  if (dims.length === 0 || mets.length === 0) {
    return {categories: [], series: []}
  }

  const catIdx = data.columns.indexOf(dims[0])
  const groupIdx = dims.slice(1).map(d => data.columns.indexOf(d))
  const metricIdx = mets.map(m => data.columns.indexOf(m))
  const multiMetric = mets.length > 1

  const categories: string[] = []
  const catSeen = new Set<string>()
  // series 名 -> (分类值 -> 待聚合数值数组)
  const seriesMap = new Map<string, Map<string, number[]>>()
  const seriesOrder: string[] = []

  for (const row of data.rows) {
    const catVal = norm(row[catIdx])
    if (!catSeen.has(catVal)) {
      catSeen.add(catVal)
      categories.push(catVal)
    }
    const groupVal = groupIdx.map(i => norm(row[i])).join(' / ')

    mets.forEach((m, j) => {
      let name: string
      if (groupVal) {
        name = multiMetric ? `${groupVal} · ${m}` : groupVal
      }
      else {
        name = m
      }
      if (!seriesMap.has(name)) {
        seriesMap.set(name, new Map())
        seriesOrder.push(name)
      }
      const cm = seriesMap.get(name)!
      if (!cm.has(catVal)) {
        cm.set(catVal, [])
      }
      const v = row[metricIdx[j]]
      if (agg === 'count') {
        if (v !== null && v !== undefined && v !== '') {
          cm.get(catVal)!.push(1)
        }
      }
      else {
        const num = typeof v === 'number' ? v : Number(v)
        if (!isNaN(num)) {
          cm.get(catVal)!.push(num)
        }
      }
    })
  }

  const series: ShapedSeries[] = seriesOrder.map(name => ({
    name,
    data: categories.map(c => {
      const vals = seriesMap.get(name)!.get(c)
      return vals && vals.length > 0 ? aggregate(vals, agg) : null
    })
  }))

  return {categories, series}
}

export interface ScatterSeries {
  name: string
  points: [number, number][]
}

/**
 * 散点数据：不聚合，逐行取 (x, y)。
 * - xField / yField：数值列
 * - groupField：可选分组列，按其值拆成多条 series
 */
export function scatterData(
  data: TableData,
  xField: string,
  yField: string,
  groupField?: string
): ScatterSeries[] {
  const xi = data.columns.indexOf(xField)
  const yi = data.columns.indexOf(yField)
  if (xi < 0 || yi < 0) {
    return []
  }
  const gi = groupField ? data.columns.indexOf(groupField) : -1

  const map = new Map<string, [number, number][]>()
  const order: string[] = []
  for (const row of data.rows) {
    const x = Number(row[xi])
    const y = Number(row[yi])
    if (isNaN(x) || isNaN(y)) {
      continue
    }
    const key = gi >= 0 ? norm(row[gi]) : yField
    if (!map.has(key)) {
      map.set(key, [])
      order.push(key)
    }
    map.get(key)!.push([x, y])
  }
  return order.map(name => ({name, points: map.get(name)!}))
}

/** 对整列做单值聚合（用于仪表盘等单值图表） */
export function aggregateColumn(data: TableData, metric: string, agg: AggKind): number {
  const mi = data.columns.indexOf(metric)
  if (mi < 0) {
    return 0
  }
  const vals: number[] = []
  for (const row of data.rows) {
    const v = row[mi]
    if (agg === 'count') {
      if (v !== null && v !== undefined && v !== '') {
        vals.push(1)
      }
      continue
    }
    const num = typeof v === 'number' ? v : Number(v)
    if (!isNaN(num)) {
      vals.push(num)
    }
  }
  return vals.length === 0 ? 0 : aggregate(vals, agg)
}

export interface TreeNode {
  name: string
  value?: number
  children?: TreeNode[]
}

interface Bucket {
  children: Map<string, Bucket>
  values: number[]
}

/**
 * 层级数据：维度按顺序嵌套，叶子值为指标聚合。
 * 供旭日图 / 矩形树图 / 树图复用。
 */
export function hierarchy(data: TableData, dims: string[], metric: string, agg: AggKind): TreeNode[] {
  const dimIdx = dims.map(d => data.columns.indexOf(d)).filter(i => i >= 0)
  const mi = data.columns.indexOf(metric)
  if (dimIdx.length === 0 || mi < 0) {
    return []
  }
  const root = new Map<string, Bucket>()
  const child = (m: Map<string, Bucket>, k: string): Bucket => {
    if (!m.has(k)) {
      m.set(k, {children: new Map(), values: []})
    }
    return m.get(k)!
  }
  for (const row of data.rows) {
    let level = root
    let node: Bucket | null = null
    for (const di of dimIdx) {
      node = child(level, norm(row[di]))
      level = node.children
    }
    const v = row[mi]
    if (agg === 'count') {
      if (v !== null && v !== undefined && v !== '') {
        node!.values.push(1)
      }
    }
    else {
      const num = typeof v === 'number' ? v : Number(v)
      if (!isNaN(num)) {
        node!.values.push(num)
      }
    }
  }
  const toNodes = (m: Map<string, Bucket>): TreeNode[] => [...m.entries()].map(([name, b]) => {
    const children = toNodes(b.children)
    if (children.length > 0) {
      return {name, children}
    }
    return {name, value: b.values.length > 0 ? aggregate(b.values, agg) : 0}
  })
  return toNodes(root)
}

const LEVEL_SEP = '::'
/** 去掉桑基/层级节点 id 的层级前缀，得到展示名 */
export function stripLevel(id: string): string {
  return id.replace(/^\d+::/, '')
}

export interface SankeyData {
  nodes: { name: string }[]
  links: { source: string; target: string; value: number }[]
}

/**
 * 桑基图数据：相邻维度之间按数据流连接，流量为指标聚合。
 * 节点 id 带层级前缀以保证唯一、避免环（展示名用 stripLevel）。
 */
export function sankeyData(data: TableData, dims: string[], metric: string, agg: AggKind): SankeyData {
  const dimIdx = dims.map(d => data.columns.indexOf(d)).filter(i => i >= 0)
  const mi = data.columns.indexOf(metric)
  if (dimIdx.length < 2 || mi < 0) {
    return {nodes: [], links: []}
  }
  const nodeSet = new Set<string>()
  const linkMap = new Map<string, number[]>()
  for (const row of data.rows) {
    for (let l = 0; l < dimIdx.length - 1; l++) {
      const sId = `${l}${LEVEL_SEP}${norm(row[dimIdx[l]])}`
      const tId = `${l + 1}${LEVEL_SEP}${norm(row[dimIdx[l + 1]])}`
      nodeSet.add(sId)
      nodeSet.add(tId)
      const key = `${sId}=>${tId}`
      if (!linkMap.has(key)) {
        linkMap.set(key, [])
      }
      const v = row[mi]
      if (agg === 'count') {
        if (v !== null && v !== undefined && v !== '') {
          linkMap.get(key)!.push(1)
        }
      }
      else {
        const num = typeof v === 'number' ? v : Number(v)
        if (!isNaN(num)) {
          linkMap.get(key)!.push(num)
        }
      }
    }
  }
  const links = [...linkMap.entries()].map(([k, vals]) => {
    const [source, target] = k.split('=>')
    return {source, target, value: vals.length > 0 ? aggregate(vals, agg) : 0}
  }).filter(l => l.value > 0)
  return {nodes: [...nodeSet].map(name => ({name})), links}
}

export interface HeatmapData {
  xCats: string[]
  yCats: string[]
  cells: [number, number, number][]
  min: number
  max: number
}

/**
 * 热力图数据：xDim × yDim 的网格，单元值为 metric 聚合。
 */
export function heatmapData(
  data: TableData,
  xDim: string,
  yDim: string,
  metric: string,
  agg: AggKind
): HeatmapData {
  const xi = data.columns.indexOf(xDim)
  const yi = data.columns.indexOf(yDim)
  const mi = data.columns.indexOf(metric)
  if (xi < 0 || yi < 0 || mi < 0) {
    return {xCats: [], yCats: [], cells: [], min: 0, max: 0}
  }

  const xCats: string[] = []
  const yCats: string[] = []
  const xIndex = new Map<string, number>()
  const yIndex = new Map<string, number>()
  const buckets = new Map<string, number[]>()

  for (const row of data.rows) {
    const xk = norm(row[xi])
    const yk = norm(row[yi])
    if (!xIndex.has(xk)) {
      xIndex.set(xk, xCats.length)
      xCats.push(xk)
    }
    if (!yIndex.has(yk)) {
      yIndex.set(yk, yCats.length)
      yCats.push(yk)
    }
    const key = `${xIndex.get(xk)}|${yIndex.get(yk)}`
    if (!buckets.has(key)) {
      buckets.set(key, [])
    }
    const v = row[mi]
    if (agg === 'count') {
      if (v !== null && v !== undefined && v !== '') {
        buckets.get(key)!.push(1)
      }
    }
    else {
      const num = typeof v === 'number' ? v : Number(v)
      if (!isNaN(num)) {
        buckets.get(key)!.push(num)
      }
    }
  }

  const cells: [number, number, number][] = []
  let min = Infinity
  let max = -Infinity
  for (const [key, vals] of buckets) {
    if (vals.length === 0) {
      continue
    }
    const [xs, ys] = key.split('|')
    const val = aggregate(vals, agg)
    cells.push([Number(xs), Number(ys), val])
    min = Math.min(min, val)
    max = Math.max(max, val)
  }
  if (!isFinite(min)) {
    min = 0
  }
  if (!isFinite(max)) {
    max = 0
  }
  return {xCats, yCats, cells, min, max}
}

export interface ParallelData {
  axes: string[]
  series: { name: string; data: number[][] }[]
}

/** 平行坐标：每行是一条跨多指标轴的折线，可选按维度分组。 */
export function parallelData(data: TableData, metrics: string[], groupDim?: string): ParallelData {
  const mIdx = metrics.map(m => data.columns.indexOf(m))
  if (metrics.length < 2 || mIdx.some(i => i < 0)) {
    return {axes: [], series: []}
  }
  const gi = groupDim ? data.columns.indexOf(groupDim) : -1
  const map = new Map<string, number[][]>()
  const order: string[] = []
  for (const row of data.rows) {
    const vals = mIdx.map(i => Number(row[i]))
    if (vals.some(isNaN)) {
      continue
    }
    const key = gi >= 0 ? norm(row[gi]) : '数据'
    if (!map.has(key)) {
      map.set(key, [])
      order.push(key)
    }
    map.get(key)!.push(vals)
  }
  return {axes: metrics, series: order.map(name => ({name, data: map.get(name)!}))}
}

export interface CandleData {
  categories: string[]
  values: number[][] // [open, close, low, high]
}

/**
 * K 线数据：按类目（日期）分组；open=首行、close=末行、low=最小、high=最大。
 */
export function candlestickData(
  data: TableData,
  categoryDim: string,
  openF: string,
  closeF: string,
  lowF: string,
  highF: string
): CandleData {
  const ci = data.columns.indexOf(categoryDim)
  const oi = data.columns.indexOf(openF)
  const cci = data.columns.indexOf(closeF)
  const li = data.columns.indexOf(lowF)
  const hi = data.columns.indexOf(highF)
  if ([ci, oi, cci, li, hi].some(i => i < 0)) {
    return {categories: [], values: []}
  }
  interface E { open: number; close: number; low: number; high: number }
  const map = new Map<string, E>()
  const order: string[] = []
  for (const row of data.rows) {
    const o = Number(row[oi])
    const c = Number(row[cci])
    const l = Number(row[li])
    const h = Number(row[hi])
    if ([o, c, l, h].some(isNaN)) {
      continue
    }
    const k = norm(row[ci])
    if (!map.has(k)) {
      map.set(k, {open: o, close: c, low: l, high: h})
      order.push(k)
    }
    else {
      const e = map.get(k)!
      e.close = c
      e.low = Math.min(e.low, l)
      e.high = Math.max(e.high, h)
    }
  }
  return {categories: order, values: order.map(k => {
    const e = map.get(k)!
    return [e.open, e.close, e.low, e.high]
  })}
}

export interface BoxplotData {
  categories: string[]
  boxes: number[][] // [min, Q1, median, Q3, max]
  outliers: [number, number][] // [catIndex, value]
}

function quantileSorted(sorted: number[], q: number): number {
  if (sorted.length === 0) {
    return 0
  }
  const pos = (sorted.length - 1) * q
  const base = Math.floor(pos)
  const rest = pos - base
  return sorted[base + 1] !== undefined ? sorted[base] + rest * (sorted[base + 1] - sorted[base]) : sorted[base]
}

/** 箱线图数据：按维度分组，对指标原始值计算五数概括与离群点。 */
export function boxplotData(data: TableData, categoryDim: string, metric: string): BoxplotData {
  const ci = data.columns.indexOf(categoryDim)
  const mi = data.columns.indexOf(metric)
  if (ci < 0 || mi < 0) {
    return {categories: [], boxes: [], outliers: []}
  }
  const groups = new Map<string, number[]>()
  const order: string[] = []
  for (const row of data.rows) {
    const num = Number(row[mi])
    if (isNaN(num)) {
      continue
    }
    const k = norm(row[ci])
    if (!groups.has(k)) {
      groups.set(k, [])
      order.push(k)
    }
    groups.get(k)!.push(num)
  }
  const boxes: number[][] = []
  const outliers: [number, number][] = []
  order.forEach((k, idx) => {
    const arr = groups.get(k)!.slice().sort((a, b) => a - b)
    const q1 = quantileSorted(arr, 0.25)
    const med = quantileSorted(arr, 0.5)
    const q3 = quantileSorted(arr, 0.75)
    const iqr = q3 - q1
    const lo = q1 - 1.5 * iqr
    const hi = q3 + 1.5 * iqr
    const inRange = arr.filter(v => v >= lo && v <= hi)
    const min = inRange.length > 0 ? inRange[0] : arr[0]
    const max = inRange.length > 0 ? inRange[inRange.length - 1] : arr[arr.length - 1]
    boxes.push([min, q1, med, q3, max])
    arr.forEach(v => {
      if (v < lo || v > hi) {
        outliers.push([idx, v])
      }
    })
  })
  return {categories: order, boxes, outliers}
}

/** 按各分类的指标合计排序并截取前 N 项（topN<=0 表示不限制） */
export function sortAndLimit(shaped: ShapedData, order: 'none' | 'asc' | 'desc', topN: number): ShapedData {
  let idx = shaped.categories.map((_, i) => i)
  if (order !== 'none') {
    const totals = idx.map(i => shaped.series.reduce((s, ser) => s + (ser.data[i] || 0), 0))
    idx = [...idx].sort((a, b) => (order === 'asc' ? totals[a] - totals[b] : totals[b] - totals[a]))
  }
  if (topN > 0) {
    idx = idx.slice(0, topN)
  }
  return {
    categories: idx.map(i => shaped.categories[i]),
    series: shaped.series.map(s => ({name: s.name, data: idx.map(i => s.data[i])}))
  }
}
