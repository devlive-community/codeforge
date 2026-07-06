// 按数据库类型构造「列」与「外键」查询。用于 schema 浏览 / ER 图 / 查询构建器。
const esc = (s: string) => s.replace(/'/g, "''")

export interface Col { name: string; type: string }
export interface Tbl { name: string; columns: Col[] }

// 标识符引用（MySQL/ClickHouse 用反引号，其余用双引号）
export const quoteIdent = (kind: string, name: string) =>
  kind === 'mysql' || kind === 'clickhouse' ? `\`${name}\`` : `"${name}"`

// 把 columnsSql 返回的 (tbl, col, typ) 行分组为按表聚合的结构
export const groupTables = (rows: any[][]): Tbl[] => {
  const map = new Map<string, Tbl>()
  for (const row of rows) {
    const tbl = String(row[0])
    if (!map.has(tbl)) {
      map.set(tbl, {name: tbl, columns: []})
    }
    map.get(tbl)!.columns.push({name: String(row[1]), type: String(row[2] ?? '')})
  }
  return [...map.values()]
}

// 返回 (tbl, col, typ) 行
export function columnsSql(kind: string, db?: string): string {
  if (kind === 'mysql') {
    return 'SELECT table_name AS tbl, column_name AS col, column_type AS typ '
      + `FROM information_schema.columns WHERE table_schema = '${esc(db || '')}' `
      + 'ORDER BY table_name, ordinal_position'
  }
  if (kind === 'postgres') {
    return 'SELECT table_name AS tbl, column_name AS col, data_type AS typ '
      + 'FROM information_schema.columns '
      + "WHERE table_schema NOT IN ('pg_catalog', 'information_schema') "
      + 'ORDER BY table_name, ordinal_position'
  }
  if (kind === 'clickhouse') {
    return 'SELECT table AS tbl, name AS col, type AS typ '
      + 'FROM system.columns WHERE database = currentDatabase() '
      + 'ORDER BY table, position'
  }
  if (kind === 'duckdb') {
    return 'SELECT table_name AS tbl, column_name AS col, data_type AS typ '
      + 'FROM information_schema.columns '
      + "WHERE table_schema NOT IN ('information_schema', 'pg_catalog') "
      + 'ORDER BY table_name, ordinal_position'
  }
  return 'SELECT m.name AS tbl, p.name AS col, p.type AS typ '
    + 'FROM sqlite_master m JOIN pragma_table_info(m.name) p '
    + "WHERE m.type IN ('table','view') AND m.name NOT LIKE 'sqlite\\_%' ESCAPE '\\' "
    + 'ORDER BY m.name, p.cid'
}

// 返回 (tbl, col, ref_table, ref_col) 行；无外键内省能力的库返回空串
export function fksSql(kind: string, db?: string): string {
  if (kind === 'mysql') {
    return 'SELECT table_name AS tbl, column_name AS col, referenced_table_name AS ref_tbl, referenced_column_name AS ref_col '
      + `FROM information_schema.key_column_usage WHERE table_schema = '${esc(db || '')}' `
      + 'AND referenced_table_name IS NOT NULL'
  }
  if (kind === 'postgres') {
    return 'SELECT tc.table_name AS tbl, kcu.column_name AS col, ccu.table_name AS ref_tbl, ccu.column_name AS ref_col '
      + 'FROM information_schema.table_constraints tc '
      + 'JOIN information_schema.key_column_usage kcu ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema '
      + 'JOIN information_schema.constraint_column_usage ccu ON ccu.constraint_name = tc.constraint_name AND ccu.table_schema = tc.table_schema '
      + "WHERE tc.constraint_type = 'FOREIGN KEY' AND tc.table_schema NOT IN ('pg_catalog', 'information_schema')"
  }
  // clickhouse / duckdb 等无外键内省，返回空
  if (kind === 'clickhouse' || kind === 'duckdb') {
    return ''
  }
  // sqlite / 内存库：用 pragma_foreign_key_list
  return 'SELECT m.name AS tbl, f."from" AS col, f."table" AS ref_tbl, f."to" AS ref_col '
    + 'FROM sqlite_master m JOIN pragma_foreign_key_list(m.name) f '
    + "WHERE m.type = 'table' AND m.name NOT LIKE 'sqlite\\_%' ESCAPE '\\'"
}
