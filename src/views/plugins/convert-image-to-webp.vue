<script setup lang="ts">
import { open, save } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { downloadDir, join } from '@tauri-apps/api/path'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { shallowRef } from 'vue'
import { writeFile, copyFile } from '@tauri-apps/plugin-fs'

type FileStatus = 'Success' | 'Error' | 'Pending'

interface File {
    path: string
    status: FileStatus
    message: string
    dest: string
}

let fileList = shallowRef<Array<File>>([])

interface Payload {
    status: FileStatus
    path: string
    message: string
    dest: string
}

const handleListener = () => {
    const current = getCurrentWebviewWindow()
    current.listen('plugin_convert_images_notify', (event) => {
        const payload = event.payload as Payload
        console.log(payload)
        fileList.value = fileList.value.map((c: File) => {
            return {
                ...c,
                status: payload.path === c.path ? payload.status : c.status,
                message: payload.message
            }
        })
    })
}

handleListener()

const getFIleName = (path: string) => {
    const tmp = path.split('\\').pop() as string
    return tmp?.split('.').slice(0, -1).join('.')
}

const download = async (data: File) => {
    const downloadDirPath = await downloadDir();
    const fileName = getFIleName(data.path)
    const target = await join(downloadDirPath, `${fileName}.webp`)
    console.log(target)
    const dest = data.dest

    const path: string | null = await save({
        title: '保存图片',
        defaultPath: target,
        filters: [
            {
                name: '图片',
                extensions: ['webp']
            }
        ]
    })
    if (path === null) return false
    console.log(path)
    await copyFile(dest, path)
}

const convertImageToWebp = async () => {
    const path: string[] | null = await open({ 
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
    if (path === null) return false
    console.log(path)
    fileList.value = [
        ...fileList.value,
        ...path.map((c) => {
            return {
                path: c,
                message: '',
                status: 'Pending',
                dest: ''
            } as File
        })
    ]

    const result = await invoke('convert_images', { paths: path })
}

const getProgressStatus = (status: FileStatus) => {
    switch (status) {
        case 'Success':
            return 'success'
        case 'Error':
            return 'exception'
        default:
            return undefined
    }
}
</script>

<template>
    <div>
        <div>
            <el-button type="primary" @click="convertImageToWebp">选择图片</el-button>
        </div>
        <div
            style="border: 1px solid var(--el-border-color); border-radius: 8px; padding: 12px 0"
            v-if="fileList.length"
        >
            <div v-for="(item, index) in fileList" :key="index">
                <el-divider v-if="index > 0" style="margin: 12px 0" />
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px 16px">
                    <div>{{ item.path }}</div>
                    <div>
                        <el-link v-if="item.status === 'Success'" type="primary" @click="download(item)">下载</el-link>
                    </div>
                </div>
                <div style="padding: 8px 16px">
                    <el-progress
                        :show-text="false"
                        :percentage="item.status === 'Pending' ? 50 : 100"
                        :status="getProgressStatus(item.status)"
                        :stroke-width="8"
                        :indeterminate="item.status === 'Pending'"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss"></style>
