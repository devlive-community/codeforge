import type {App, Directive} from 'vue'

// 全局提示气泡：任何带原生 title（或 data-cf-tip）的元素，悬停时显示统一样式的气泡，
// 并抑制浏览器默认 title。一处实现，全应用生效（含动态/未来新增的元素）。
// 样式与 ui/Tooltip.vue 保持一致（深色气泡、白字、圆角、出现在元素下方）。

let tip: HTMLDivElement | null = null

function ensureTip(): HTMLDivElement {
    if (tip) {
        return tip
    }
    tip = document.createElement('div')
    tip.setAttribute('role', 'tooltip')
    Object.assign(tip.style, {
        position: 'fixed',
        zIndex: '99999',
        pointerEvents: 'none',
        background: '#1f2937',
        color: '#ffffff',
        fontSize: '12px',
        lineHeight: '1.3',
        padding: '4px 8px',
        borderRadius: '6px',
        boxShadow: '0 4px 14px rgba(0,0,0,0.25)',
        maxWidth: '320px',
        opacity: '0',
        transition: 'opacity 120ms ease',
        top: '0',
        left: '0'
    } as Partial<CSSStyleDeclaration>)
    document.body.appendChild(tip)
    return tip
}

function show(el: HTMLElement, text: string) {
    const t = ensureTip()
    t.textContent = text
    t.style.whiteSpace = text.length > 40 ? 'normal' : 'nowrap'
    t.style.opacity = '0'
    const r = el.getBoundingClientRect()
    const bw = t.offsetWidth
    const bh = t.offsetHeight
    let left = r.left + r.width / 2 - bw / 2
    let top = r.bottom + 6
    // 下方放不下则翻到上方
    if (top + bh > window.innerHeight - 4) {
        top = r.top - bh - 6
    }
    left = Math.max(4, Math.min(left, window.innerWidth - bw - 4))
    t.style.left = `${Math.round(left)}px`
    t.style.top = `${Math.round(top)}px`
    t.style.opacity = '1'
}

function hide() {
    if (tip) {
        tip.style.opacity = '0'
    }
}

// 读取提示文案：首次把原生 title 迁移到 data-cf-tip 并移除 title（抑制浏览器默认气泡），
// 之后从 data-cf-tip 读取；title 再次出现（动态更新）时会重新捕获。
function getText(el: HTMLElement): string | null {
    const title = el.getAttribute('title')
    if (title) {
        el.setAttribute('data-cf-tip', title)
        el.removeAttribute('title')
        return title
    }
    return el.getAttribute('data-cf-tip')
}

function onOver(e: Event) {
    const target = e.target as HTMLElement | null
    if (!target || !target.closest) {
        return
    }
    const el = target.closest('[title],[data-cf-tip]') as HTMLElement | null
    if (!el) {
        return
    }
    const text = getText(el)
    if (text) {
        show(el, text)
    }
}

function onOut(e: Event) {
    const target = e.target as HTMLElement | null
    if (!target || !target.closest) {
        return
    }
    const el = target.closest('[title],[data-cf-tip]') as HTMLElement | null
    if (!el) {
        return
    }
    // 移动到该元素的子节点内不应隐藏
    const related = (e as MouseEvent).relatedTarget as Node | null
    if (related && el.contains(related)) {
        return
    }
    hide()
}

export const TooltipPlugin = {
    install(_app: App) {
        if (typeof document === 'undefined') {
            return
        }
        document.addEventListener('mouseover', onOver, true)
        document.addEventListener('mouseout', onOut, true)
        document.addEventListener('scroll', hide, true)
        window.addEventListener('blur', hide)
    }
}

// 可选的显式用法：v-tooltip="'文本'"（无需依赖原生 title）
export const vTooltip: Directive<HTMLElement, string | undefined> = {
    mounted(el, binding) {
        if (binding.value) {
            el.setAttribute('data-cf-tip', binding.value)
            el.removeAttribute('title')
        }
    },
    updated(el, binding) {
        if (binding.value) {
            el.setAttribute('data-cf-tip', binding.value)
        }
    }
}
