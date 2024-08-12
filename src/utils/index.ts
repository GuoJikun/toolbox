import { Window, type WindowLabel } from '@tauri-apps/api/window'
import { register, isRegistered, unregister } from '@tauri-apps/plugin-global-shortcut'
import { Command } from '@tauri-apps/plugin-shell'
import { join, resolveResource } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'

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
 * 转化路径为符合当前操作系统的路径
 * @param path 路径
 * @returns
 */
export const formatPath = async (...path: string[]) => {
    return await join(...path)
}

// 初始化一个 caddy 文件服务器
export const initHttpServer = async () => {
    const staticDirPath = await resolveResource('plugins')
    console.log('staticDirPath', staticDirPath)
    const command = Command.sidecar('binaries/caddy', [
        'file-server',
        '--listen',
        'localhost:6543',
        '--root',
        staticDirPath
    ])
    const output = await command.execute()
    console.log('output', output)
}

/**执行本机应用程序 */
export const runSoftware = async (path: string) => {
    // Command.create(path).execute()
    const res = await invoke('run_external_program', { executablePath: path, args: [] })

    console.log(res)
}
