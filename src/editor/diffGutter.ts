import {gutter, GutterMarker, EditorView} from '@codemirror/view'
import {StateEffect, StateField, RangeSet, RangeSetBuilder} from '@codemirror/state'

export type LineKind = 'add' | 'mod'

export interface DiffMarkers
{
    // 当前文档行号(1-based) → 标记类型
    changed: Map<number, LineKind>
    // 在这些行之前发生了删除（显示三角形提示）
    deleted: Set<number>
}

// 设置差异标记的 effect（由外部计算后派发）
export const setDiffMarkers = StateEffect.define<DiffMarkers>()

class DiffGutterMarker extends GutterMarker
{
    constructor(readonly cls: string)
    {
        super()
    }

    eq(other: DiffGutterMarker)
    {
        return other.cls === this.cls
    }

    toDOM()
    {
        const el = document.createElement('div')
        el.className = this.cls
        return el
    }
}

const addMarker = new DiffGutterMarker('cm-diff-add')
const modMarker = new DiffGutterMarker('cm-diff-mod')
const delMarker = new DiffGutterMarker('cm-diff-del')

// 由 DiffMarkers 构建定位到各行起点的 RangeSet
const buildSet = (state: any, data: DiffMarkers): RangeSet<GutterMarker> => {
    const lineCount = state.doc.lines
    // 按行号排序后写入，RangeSetBuilder 要求位置递增
    const entries: { line: number, marker: GutterMarker }[] = []
    for (const [line, kind] of data.changed) {
        if (line >= 1 && line <= lineCount) {
            entries.push({line, marker: kind === 'add' ? addMarker : modMarker})
        }
    }
    for (const line of data.deleted) {
        if (line >= 1 && line <= lineCount && !data.changed.has(line)) {
            entries.push({line, marker: delMarker})
        }
    }
    entries.sort((a, b) => a.line - b.line)

    const builder = new RangeSetBuilder<GutterMarker>()
    for (const e of entries) {
        const from = state.doc.line(e.line).from
        builder.add(from, from, e.marker)
    }
    return builder.finish()
}

const diffField = StateField.define<RangeSet<GutterMarker>>({
    create: () => RangeSet.empty,
    update(set, tr) {
        // 文档变化时先随之平移，保证下次重算前位置不至于错乱
        set = set.map(tr.changes)
        for (const e of tr.effects) {
            if (e.is(setDiffMarkers)) {
                set = buildSet(tr.state, e.value)
            }
        }
        return set
    }
})

const diffGutterTheme = EditorView.baseTheme({
    '.cm-diff-gutter .cm-gutterElement': {
        padding: '0',
    },
    '.cm-diff-add, .cm-diff-mod, .cm-diff-del': {
        width: '3px',
        height: '100%',
        marginLeft: '2px',
    },
    '.cm-diff-add': {background: '#2ea043'},
    '.cm-diff-mod': {background: '#d29922'},
    // 删除：用红色小三角提示
    '.cm-diff-del': {
        background: 'transparent',
        width: '0',
        height: '0',
        marginLeft: '1px',
        borderLeft: '4px solid #f85149',
        borderTop: '4px solid transparent',
        borderBottom: '4px solid transparent',
    },
})

const diffGutterView = gutter({
    class: 'cm-diff-gutter',
    markers: v => v.state.field(diffField),
})

// 编辑器差异标记扩展（默认无标记，由外部 dispatch setDiffMarkers 填充）
export const diffGutterExtension = [diffField, diffGutterView, diffGutterTheme]

// 基于 LCS 的逐行差异，输出当前文档各行的标记
export const computeDiffMarkers = (baseline: string, current: string): DiffMarkers => {
    const a = baseline.length ? baseline.split('\n') : []
    const b = current.length ? current.split('\n') : []
    const n = a.length, m = b.length
    const changed = new Map<number, LineKind>()
    const deleted = new Set<number>()

    // 空基线：全部视为新增
    if (n === 0) {
        for (let i = 1; i <= m; i++) {
            changed.set(i, 'add')
        }
        return {changed, deleted}
    }

    // 超大文件不做精细 diff，避免 O(n*m)
    if (n * m > 4_000_000) {
        return {changed, deleted}
    }

    const dp: number[][] = Array.from({length: n + 1}, () => new Array(m + 1).fill(0))
    for (let i = n - 1; i >= 0; i--) {
        for (let j = m - 1; j >= 0; j--) {
            dp[i][j] = a[i] === b[j] ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1])
        }
    }

    let i = 0, j = 0
    let curLine = 0
    let pendingDel = 0
    const step = (op: 'same' | 'add' | 'del') => {
        if (op === 'del') {
            pendingDel++
            return
        }
        if (op === 'add') {
            curLine++
            if (pendingDel > 0) {
                changed.set(curLine, 'mod')
                pendingDel--
            }
            else {
                changed.set(curLine, 'add')
            }
            return
        }
        // same
        curLine++
        if (pendingDel > 0) {
            deleted.add(curLine)
            pendingDel = 0
        }
    }

    while (i < n && j < m) {
        if (a[i] === b[j]) {
            step('same'); i++; j++
        }
        else if (dp[i + 1][j] >= dp[i][j + 1]) {
            step('del'); i++
        }
        else {
            step('add'); j++
        }
    }
    while (i < n) {
        step('del'); i++
    }
    while (j < m) {
        step('add'); j++
    }
    // 文件末尾的删除：标记在最后一行
    if (pendingDel > 0) {
        deleted.add(curLine > 0 ? curLine : 1)
    }

    return {changed, deleted}
}
