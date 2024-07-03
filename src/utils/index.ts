import { Window, type WindowLabel } from "@tauri-apps/api/window";
import { register, isRegistered, unregister } from "@tauri-apps/plugin-global-shortcut";


export const getWindow = (label: WindowLabel) => {
    const windows = Window.getAll();
    return windows.find((window) => window.label === label);
}

export const registerShortcut = async (shortcut:string, callback:() => void) => {
    const registered = await isRegistered(shortcut);
    if(registered){
        if(import.meta.env.DEV){
            await unregister(shortcut)
        }else{
            console.log('Shortcut is already registered');
            return
        }
    }
    
    
    // 注册快捷键
    try {
        await register(shortcut, callback);
        console.log('Shortcut registered successfully');
    } catch (error) {
        console.error('Failed to register shortcut', error);
    }
}
