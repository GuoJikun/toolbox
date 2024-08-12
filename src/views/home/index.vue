<script setup lang="ts">
import InstallPlugin from '@/components/install-plugin.vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

const handlerInstallSuccess = (path: string) => {
    console.log('path', path)
}

const handleBeforePermission = () => {
    console.log('添加权限前')
    const currentWindow = getCurrentWindow()
    currentWindow.isMaximized().then((res) => {
        console.log('isMaximized', res)
    })
}
const handlePermission = () => {
    console.log('添加权限')
    invoke('add_capabilities', {
        window: 'main',
        webview: '',
        permissions: ['window:allow-is-maximize']
    }).then((res) => {
        console.log('add_capabilities', res)
    })
}

const handleAfterPermission = () => {
    console.log('添加权限后')
    const currentWindow = getCurrentWindow()
    currentWindow.isMaximized().then((res) => {
        console.log('isMaximized', res)
    })
}
</script>
<template>
    <div class="home">
        <h1>Home Index</h1>
        <el-space>
            <el-button @click="handleBeforePermission">添加权限前</el-button>
            <el-button @click="handlePermission">添加权限</el-button>
            <el-button @click="handleAfterPermission">添加权限后</el-button>
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
