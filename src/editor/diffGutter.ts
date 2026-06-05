import {Decoration, DecorationSet, EditorView} from '@codemirror/view'
import {StateEffect, StateField, RangeSetBuilder} from '@codemirror/state'

export type LineKind = 'add' | 'mod'

export interface DiffMarkers
{
    // 当前文档行号(1-based) → 标记类型
    changed: Map<number, LineKind>
    // 在这些行之前发生了删除（在行首显示红色小三角）
    deleted: Set<number>
}

// 设置差异标记的 effect（由外部计算后派发）
export const setDiffMarkers = StateEffect.define<DiffMarkers>()

// 行装饰：在行左缘绘制彩色竖条（不新增 gutter 列，避免与行号列冲突）
// 新增=绿、修改=琥珀；删除在相邻行底缘用琥珀虚线提示，避免绝对定位三角影响行高
const addLine = Decoration.line({class: 'cm-diff-add'})
const modLine = Decoration.line({class: 'cm-diff-mod'})
const delLine = Decoration.line({class: 'cm-diff-del'})

const buildSet = (state: any, data: DiffMarkers): DecorationSet => {
    const lineCount = state.doc.lines
    const builder = new RangeSetBuilder<Decoration>()
    // 按行号升序写入，满足 RangeSetBuilder 的递增要求
    for (let n = 1; n <= lineCount; n++) {
        const from = state.doc.line(n).from
        if (data.changed.has(n)) {
            builder.add(from, from, data.changed.get(n) === 'add' ? addLine : modLine)
        }
        else if (data.deleted.has(n)) {
            builder.add(from, from, delLine)
        }
    }
    return builder.finish()
}

const diffField = StateField.define<DecorationSet>({
    create: () => Decoration.none,
    update(set, tr) {
        // 文档变化时随之平移，下一次重算前位置不至错乱
        set = set.map(tr.changes)
        for (const e of tr.effects) {
            if (e.is(setDiffMarkers)) {
                set = buildSet(tr.state, e.value)
            }
        }
        return set
    },
    provide: f => EditorView.decorations.from(f)
})

const diffTheme = EditorView.baseTheme({
    // 左缘竖条：新增=绿、修改=琥珀
    '.cm-diff-add': {boxShadow: 'inset 2px 0 0 0 #2ea043'},
    '.cm-diff-mod': {boxShadow: 'inset 2px 0 0 0 #d29922'},
    // 删除：该行底缘一条红线（提示其下方有内容被删除），不影响行高
    '.cm-diff-del': {boxShadow: 'inset 0 -2px 0 0 #f85149'},
})

// 编辑器差异标记扩展（默认无标记，由外部 dispatch setDiffMarkers 填充）
export const diffGutterExtension = [diffField, diffTheme]

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
