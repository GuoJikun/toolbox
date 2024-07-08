import { Window, type WindowLabel } from '@tauri-apps/api/window'
import { register, isRegistered, unregister } from '@tauri-apps/plugin-global-shortcut'

import { exists, mkdir } from '@tauri-apps/plugin-fs'
import { homeDir, join } from '@tauri-apps/api/path'

export const getWindow = (label: WindowLabel) => {
    const windows = Window.getAll()
    return windows.find((window) => window.label === label)
}

export const registerShortcut = async (shortcut: string, callback: () => void) => {
    const registered = await isRegistered(shortcut)
    if (registered) {
        if (import.meta.env.DEV) {
            await unregister(shortcut)
        } else {
            console.log('Shortcut is already registered')
            return
        }
    }

    // 注册快捷键
    try {
        await register(shortcut, callback)
        console.log('Shortcut registered successfully')
    } catch (error) {
        console.error('Failed to register shortcut', error)
    }
}

/**
 *  获取插件根目录
 * @returns 插件目录
 */
export const getPluginsPath = async () => {
    const homePath = await homeDir()
    const pluginDir = await join(homePath, '.vtools/plugins')
    return pluginDir
}
/**
 * 获取插件路径
 * @param name 插件名称/文件夹名称
 * @returns
 */
export const getPluginPath = async (name: string) => {
    const pluginsDir = await getPluginsPath()
    return join(pluginsDir, name)
}
/**
 * 初始化插件目录
 */
export const initPluginDir = async () => {
    const pluginDir = await getPluginsPath()
    const pluginDirExists = await exists(pluginDir)
    if (!pluginDirExists) {
        await mkdir(pluginDir, { recursive: true })
    }
}

/**
 * 转化路径为符合当前操作系统的路径
 * @param path 路径
 * @returns
 */
export const formatPath = async (...path: string[]) => {
    return await join(...path)
}
