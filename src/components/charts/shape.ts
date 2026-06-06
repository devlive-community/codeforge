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

/**
 * 按维度分组聚合指标。
 * - dimension：分类列；同一维度值的多行会被聚合
 * - metrics：一个或多个数值列，每个生成一条 series
 * - agg：聚合方式；count 时统计非空行数（与具体数值无关）
 */
export function aggregateByDimension(
  data: TableData,
  dimension: string,
  metrics: string[],
  agg: AggKind
): ShapedData {
  const dimIdx = data.columns.indexOf(dimension)
  const metricIdx = metrics.map(m => data.columns.indexOf(m))
  if (dimIdx < 0 || metricIdx.some(i => i < 0)) {
    return {categories: [], series: []}
  }

  const order: string[] = []
  // 维度值 -> 每个指标的数值数组
  const buckets = new Map<string, number[][]>()

  for (const row of data.rows) {
    const raw = row[dimIdx]
    const key = raw === null || raw === undefined ? '(空)' : String(raw)
    if (!buckets.has(key)) {
      buckets.set(key, metricIdx.map(() => []))
      order.push(key)
    }
    const slot = buckets.get(key)!
    metricIdx.forEach((mi, j) => {
      const v = row[mi]
      if (agg === 'count') {
        if (v !== null && v !== undefined && v !== '') {
          slot[j].push(1)
        }
        return
      }
      const num = typeof v === 'number' ? v : Number(v)
      if (!isNaN(num)) {
        slot[j].push(num)
      }
    })
  }

  const series: ShapedSeries[] = metrics.map((name, j) => ({
    name: agg === 'count' ? `${name}(计数)` : name,
    data: order.map(key => {
      const vals = buckets.get(key)![j]
      return vals.length === 0 ? null : aggregate(vals, agg)
    })
  }))

  return {categories: order, series}
}
