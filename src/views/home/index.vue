<script setup lang="ts">
import InstallPlugin from '@/components/install-plugin.vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@/utils/index'

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
</script>
<template>
    <div class="home">
        <h1>Home Index</h1>
        <el-space>
            <el-button @click="getScreenshot">获取屏幕截图</el-button>
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
