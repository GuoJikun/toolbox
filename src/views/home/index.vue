<script setup lang="ts">
import InstallPlugin from '@/components/install-plugin.vue'
import { invoke } from '@tauri-apps/api/core'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { execModulePlugin } from '@/utils/plugin'

const handlerInstallSuccess = (path: string) => {
    console.log('path', path)
}

const getScreenshot = async () => {
    const path = await invoke('screenshot_desktop')
    console.log('res', path)
    if (!path) {
        console.log('截图失败')
        return
    }
}

const openScreenshotWindow = async () => {
    const pluginConfig = {
        type: 'module',
        name: '截图',
        id: 'screenshot',
        main: 'index.html',
        devMain: 'http://localhost:6543/screenshot/index.html',
        version: '0.1.0',
        author: 'Toolbox',
        homepage: 'https://github.com/GuoJikun/toolbox-plugin',
        description: '截图功能',
        primissions: [],
        keywords: ['screenshot', '截图'],
        windowConfig: {
            fullscreen: false,
            alwaysOnTop: false,
            skipTaskbar: false,
            focus: true,
            decorations: false,
            resizable: true,
            transparent: false
        },
        scriptEnv: null
    }

    let indexPath = `http://localhost:5173/`

    console.log('indexPath', indexPath)
    execModulePlugin(indexPath, pluginConfig)
}
</script>
<template>
    <div class="home">
        <h1>Home Index</h1>
        <el-space>
            <el-button @click="getScreenshot">获取屏幕截图</el-button>
            <el-button @click="openScreenshotWindow">打开截图功能</el-button>
        </el-space>

        <div>
            <InstallPlugin tag="button" @confirm="handlerInstallSuccess"> 选择目录 </InstallPlugin>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.home {
    padding: 12px;
}
</style>
