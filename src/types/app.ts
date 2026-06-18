export interface ExecutionResult
{
    id?: number
    success: boolean
    code: string
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
    font_family?: string
    show_line_numbers?: boolean
    show_function_help?: boolean
    space_dot_omission?: boolean
    show_minimap?: boolean
    show_sticky_scroll?: boolean
    layout?: LayoutMode
    last_direction?: SplitDirection
    max_open_file_size?: number
    run_save_strategy?: RunSaveStrategy
}

export type RunSaveStrategy = 'auto-save' | 'ask' | 'temp-copy'

export type SplitDirection = 'horizontal' | 'vertical'

export type LayoutMode = SplitDirection | 'editor'

export interface EnvironmentVersion
{
    version: string
    download_url: string
    install_path: string | null
    is_installed: boolean
    size: number | null
    release_date: string | null
}

export interface EnvironmentInfo
{
    language: string
    current_version: string | null
    installed_versions: EnvironmentVersion[]
    available_versions: EnvironmentVersion[]
    error?: string | null  // 错误信息（如获取可用版本失败）
}

export interface DownloadProgress
{
    language: string
    version: string
    downloaded: number
    total: number
    percentage: number
    status: 'downloading' | 'extracting' | 'installing' | 'completed' | 'failed'
}
