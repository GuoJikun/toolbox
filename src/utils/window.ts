import { Window, type WindowLabel } from '@tauri-apps/api/window'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { error } from '@tauri-apps/plugin-log'

export const getWindow = async (label: WindowLabel) => {
    return await Window.getByLabel(label)
}

export const createWebviewWindow = async (label: WindowLabel, config = {}) => {
    const conf = {
        title: 'defaultLabel',
        width: 1000,
        height: 600,
        resizable: true,
        center: true,
        ...config
    }
    const webview = new WebviewWindow(label, conf)
    await webview.once('tauri://created', async (e) => {
        await webview.show()
    })

    await webview.once('tauri://error', async (e) => {
        await error(`webviewWindow label 创建失败，错误信息：${JSON.stringify(e)}`)
    })
}

export const getWebviewWindow = async (label: WindowLabel) => {
    return await WebviewWindow.getByLabel(label)
}