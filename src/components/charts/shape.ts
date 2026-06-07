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
