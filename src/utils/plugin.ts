import type { PluginConfig, ScriptEnv } from '@/utils/typescript'
import { invoke } from '@tauri-apps/api/core'
import { Window } from '@tauri-apps/api/window'
import { Webview, type WebviewOptions } from '@tauri-apps/api/webview'
import { exists, mkdir } from '@tauri-apps/plugin-fs'
import { homeDir, join } from '@tauri-apps/api/path'
import { getWindow } from './window'

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

export const execScriptPlugin = async (env: ScriptEnv, path: string, args: string[] = []) => {
    let fn = ''
    switch (env) {
        case 'nodejs':
            fn = 'run_node_script'
            break
        case 'php':
            fn = 'run_php_script'
            break
        case 'python':
            fn = 'run_python_script'
            break
        default:
            fn = 'run_node_script'
            break
    }

    return new Promise((resolve, reject) => {
        invoke(fn, { script: path, args })
            .then((result) => {
                resolve(result)
            })
            .catch((err) => {
                reject(err)
            })
    })
}

export const execModulePlugin = async (url: string, pluginConfig: PluginConfig) => {
    const windowLabel = `toolbox-plugin-window-${pluginConfig.id}`
    let currentWindow = getWindow(windowLabel)
    if (!currentWindow) {
        currentWindow = new Window(windowLabel, {
            center: true,
            width: 1000,
            height: 600
        })
    }
    currentWindow.listen('tauri://window-created', () => {
        console.log('tauri://window-created')
    })
    const webviewLabel = `toolbox-plugin-webview-${pluginConfig.id}`
    const webviewOption: WebviewOptions = {
        url: url,
        width: 1000,
        height: 600,
        x: 0,
        y: 0
    }
    const webview = new Webview(currentWindow, webviewLabel, webviewOption)

    webview.listen('tauri://webview-created', () => {
        console.log('webview-created')
    })
    webview.once('tauri://error', function (e) {
        // an error happened creating the webview
        console.log('error', e)
    })

    console.log('execModulePlugin', webview)
    currentWindow.show()
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
