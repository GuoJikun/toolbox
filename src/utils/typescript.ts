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
    type: 'binary',
    prefix?: string
}

interface PluginModuleConfig extends PluginBaseConfig {
    type: 'module'
    shortcut?: string
    permissions: string[]
    windowConfig: WindowOptions
}

export type ScriptEnv = 'node' | 'php' | 'python'
interface PluginScriptConfig extends PluginBaseConfig {
    type: 'script'
    scriptEnv: ScriptEnv
    prefix?: string
}

export type PluginType = "script" | "module" | "binary"
export type PluginConfig = PluginBinaryConfig | PluginModuleConfig | PluginScriptConfig

export interface InstalledPkg {
    name: string
    path: string
    icon: string
}

export interface InputFormater {
    prefix: string
    value: Array<string>
}
