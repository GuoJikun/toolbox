import { type WindowOptions } from '@tauri-apps/api/window'

export interface PluginBaseConfig {
    logo?: string
    id: string
    name: string
    description: string
    version: string
    enable?: boolean
    author?: string
    email?: string
    homepage?: string
    keywords?: string[]
    main: string
}

interface PluginBinaryConfig extends PluginBaseConfig {
    prefix?: string
}

interface PluginModuleConfig extends PluginBaseConfig {
    shortcut?: string
    primissions: string[]
    windowConfig: WindowOptions
}

export type ScriptEnv = 'node' | 'php' | 'python'
interface PluginScriptConfig extends PluginBaseConfig {
    scriptEnv: ScriptEnv
    prefix?: string
}

export interface PluginConfig extends PluginBinaryConfig, PluginModuleConfig, PluginScriptConfig {
    type: 'script' | 'binary' | 'module'
}

export interface InstalledPkg {
    name: string
    path: string
    icon: string
}

export interface InputFormater {
    prefix: string
    value: Array<string>
}
