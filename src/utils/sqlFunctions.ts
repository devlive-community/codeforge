// SQL 函数建议表（仅作输入建议，用户可自由输入任意函数名 + 参数）
export interface FuncDef { name: string; cat: string }

export const FUNC_CATS = ['agg', 'string', 'date', 'math', 'convert']

export const FUNC_DEFS: FuncDef[] = [
  // 聚合
  {name: 'COUNT', cat: 'agg'}, {name: 'COUNT DISTINCT', cat: 'agg'}, {name: 'SUM', cat: 'agg'},
  {name: 'AVG', cat: 'agg'}, {name: 'MIN', cat: 'agg'}, {name: 'MAX', cat: 'agg'}, {name: 'GROUP_CONCAT', cat: 'agg'},
  // 字符串
  {name: 'UPPER', cat: 'string'}, {name: 'LOWER', cat: 'string'}, {name: 'LENGTH', cat: 'string'},
  {name: 'TRIM', cat: 'string'}, {name: 'LTRIM', cat: 'string'}, {name: 'RTRIM', cat: 'string'}, {name: 'REVERSE', cat: 'string'},
  {name: 'SUBSTRING', cat: 'string'}, {name: 'REPLACE', cat: 'string'}, {name: 'CONCAT', cat: 'string'},
  {name: 'LEFT', cat: 'string'}, {name: 'RIGHT', cat: 'string'}, {name: 'SPLIT_PART', cat: 'string'},
  // 日期时间
  {name: 'DATE', cat: 'date'}, {name: 'YEAR', cat: 'date'}, {name: 'MONTH', cat: 'date'}, {name: 'DAY', cat: 'date'},
  {name: 'HOUR', cat: 'date'}, {name: 'MINUTE', cat: 'date'}, {name: 'SECOND', cat: 'date'},
  {name: 'DATE_TRUNC', cat: 'date'}, {name: 'EXTRACT', cat: 'date'},
  // 数学
  {name: 'ABS', cat: 'math'}, {name: 'ROUND', cat: 'math'}, {name: 'CEIL', cat: 'math'}, {name: 'FLOOR', cat: 'math'},
  {name: 'SQRT', cat: 'math'}, {name: 'SIGN', cat: 'math'}, {name: 'MOD', cat: 'math'}, {name: 'POWER', cat: 'math'},
  // 转换 / 空值
  {name: 'CAST', cat: 'convert'}, {name: 'COALESCE', cat: 'convert'}, {name: 'NULLIF', cat: 'convert'}
]

// 把「函数 + 列表达式 + 额外参数」拼成 SQL 片段
// expr 为已生成的列引用（可能带表限定）；fn 为空时原样返回
export const wrapFunc = (expr: string, fn?: string, args?: string): string => {
  if (!fn) {
    return expr
  }
  if (fn === 'COUNT DISTINCT') {
    return `COUNT(DISTINCT ${expr})`
  }
  const extra = (args || '').trim()
  return `${fn}(${expr}${extra ? ', ' + extra : ''})`
}
