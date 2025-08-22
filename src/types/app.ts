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
