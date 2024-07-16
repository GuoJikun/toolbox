import { type WindowOptions } from '@tauri-apps/api/window'

export interface Plugin {
    logo?: string | URL
    id: string
    name: string
    description: string
    version: string
    enable?: boolean
    author?: string
    email?: string
}

interface PluginBinaryConfig {
    id: string
    name: string
    main: string
    version: string
    description: string
    logo?: string
    author?: string
    email?: string
    homepage?: string
    prefix?: string
}

interface PluginModuleConfig {
    id: string
    name: string
    main: string
    version: string
    description: string
    logo?: string
    primissions: string[]
    author?: string
    email?: string
    homepage?: string
    keywords?: string[]
    shortcut?: string
    windowConfig: WindowOptions
}

export type ScriptEnv = 'nodejs' | 'php' | 'python'
interface PluginScriptConfig {
    id: string
    name: string
    main: string
    scriptEnv: ScriptEnv
    version: string
    description: string
    logo?: string
    author?: string
    email?: string
    homepage?: string
    keywords?: string[]
    prefix?: string
}

export interface PluginConfig extends PluginBinaryConfig, PluginModuleConfig, PluginScriptConfig {
    type: 'script' | 'binary' | 'module'
}
