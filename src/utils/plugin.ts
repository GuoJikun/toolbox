import type { PluginConfig } from '@/utils/typescript'
import { invoke } from '@tauri-apps/api/core'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { exists, mkdir } from '@tauri-apps/plugin-fs'
import { homeDir, join } from '@tauri-apps/api/path'

/**
 * 获取插件根目录
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
 * 执行二进制插件
 * @param path 二进制文件路径
 * @param args 参数
 */
export const execBinaryPlugin = async (executablePath: string, args: string[] = []) => {
    return new Promise((resolve, reject) => {
        invoke('run_external_program', { executablePath, args })
            .then((result) => {
                resolve(result)
            })
            .catch((err) => {
                reject(err)
            })
    })
}

export const execScriptPlugin = async (path: string, env: string) => {}

export const execModulePlugin = async (url: string, pluginConfig: PluginConfig) => {
    const webview = new WebviewWindow(`toolbox-plugin-${pluginConfig.id}`, {
        url: url
    })
    console.log('webview', webview)
    webview.show()
}

/**
 * 根据唯一标识获取插件
 * @param prefix 执行程序的唯一标识
 * @param pluginList 插件列表
 * @returns
 */
export const getPluginOfPrefix = (prefix: string, pluginList: Array<PluginConfig> = []) => {
    return pluginList.find((plugin) => plugin.prefix === prefix)
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
