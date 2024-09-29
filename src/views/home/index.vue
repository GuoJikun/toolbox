<script setup lang="ts">
import InstallPlugin from '@/components/install-plugin.vue'
import { invoke } from '@tauri-apps/api/core'
import { execModulePlugin } from '@/utils/plugin'
import { useRouter, type RouterOptions } from "vue-router"

const handlerInstallSuccess = (path: string) => {
    console.log('path', path)
}

const getScreenshot = async () => {
    const pixelData = await invoke('screenshot_desktop')
    console.log('pixelData', pixelData)
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
            fullscreen: true,
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
const router = useRouter()
const routeTo = (route: string) => {
    router.push(route);
}
</script>
<template>
    <div class="home">
        <h1>Home Index</h1>
        <el-space>
            <el-button @click="getScreenshot">获取屏幕截图</el-button>
            <el-button @click="openScreenshotWindow">打开截图功能</el-button>
        </el-space>
        <el-divider/>
        <el-space>
            <el-button @click="routeTo('/plugins/color-conversion')">颜色转换</el-button>
        </el-space>
        <el-divider/>
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
