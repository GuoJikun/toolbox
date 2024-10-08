import { Window, type WindowLabel } from '@tauri-apps/api/window'

export const getWindow = async (label: WindowLabel) => {
    const windows = await Window.getByLabel(label)
    return windows
}
