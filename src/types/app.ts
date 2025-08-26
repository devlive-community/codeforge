export interface ExecutionResult
{
    success: boolean
    stdout: string
    stderr: string
    execution_time: number
    timestamp: number
    language: string
}

export interface LanguageInfo
{
    installed: boolean
    version: string
    path: string
    language: string
}

export interface EnvInfo
{
    installed: boolean
    version: string
    path: string
    language: string
}

export interface Language
{
    name: string
    value: string
}

export interface CodeOutputEvent
{
    type: 'stdout' | 'stderr'
    content: string
    language: string
}

export interface AppInfo
{
    version: string
    build_time: string
    platform: string
    arch: string
}

export interface EditorConfig
{
    theme?: string
    indent_with_tab?: boolean
    tab_size?: number
    font_size?: number
    show_line_numbers?: boolean
    show_function_help?: boolean
    space_dot_omission?: boolean
}
