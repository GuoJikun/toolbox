<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog"
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { shallowRef } from 'vue'

interface File {
    path: string,
    status?: 'success' | 'error' | 'pending'
    message?: string
}

let fileList = shallowRef<Array<File>>([])

interface Payload {
    status: 'success' | 'error' | 'pending',
    path: string
    message: string
}
getCurrentWebviewWindow().listen('plugin_convert_images_notify', (event) => {
    const payload = event.payload as Payload
    console.log(payload)
    fileList.value = fileList.value.map((c:File) => {
        return {
            ...c,
            status: payload.path === c.path ? payload.status : c.status,
            message: payload.message
        }
    })
})

const convertImageToWebp = async () => {
    const path: string[] = await open({
        title: '选择图片',
        directory: false,
        multiple: true,
        filters: [
            {
                name: '图片',
                extensions: ['png', 'jpg', 'jpeg', 'apng']
            }
        ]
    })

    console.log(path)
    fileList.value = path.map(c => {
        return {
            path: c,
            message: '',
            status: 'pending'
        }
    })

    const result = await invoke('convert_image', path)
}
</script>

<template>

</template>

<style scoped lang="scss">

</style>