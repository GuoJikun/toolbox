import { Window, type WindowLabel } from '@tauri-apps/api/window'

export const getWindow = (label: WindowLabel) => {
    const windows = Window.getAll()
    return windows.find((window) => window.label === label)
}
