<script setup lang="ts">
import InstallPlugin from '@/components/install-plugin.vue'
import { execModulePlugin } from '@/utils/plugin'

const handlerInstallSuccess = (path: string) => {
    console.log('path', path)
}

const handleWebview = () => {
    execModulePlugin('/search', {
        type: 'module',
        id: 'calc',
        name: '计算器',
        main: 'index.html',
        version: '0.1.0',
        primissions: [],
        description: '计算器',
        scriptEnv: 'node'
    })
}
</script>
<template>
    <div class="home">
        <h1>Home Index</h1>
        <el-space>
            <router-link to="/setting">设置</router-link>
            <router-link to="/plugins">插件</router-link>
            <el-button @click="handleWebview">打开 Webview 窗口</el-button>
        </el-space>

        <div>
            <InstallPlugin tag="button" @confirm="handlerInstallSuccess"> 选择 目录 </InstallPlugin>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.home {
    padding: 12px;
}
</style>
