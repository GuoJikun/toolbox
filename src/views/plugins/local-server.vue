<script setup lang="ts">
import { ref } from 'vue'
import { useThrottleFn } from '@vueuse/core'
import { ElMessage } from 'element-plus'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

const path = ref<string>('')

const chooseDir = () => {
    open({
        title: '选择共享目录',
        directory: true,
        multiple: false
    }).then((res) => {
        if (res) {
            path.value = res
        }
    })
}
const loading = ref<boolean>(false)
const msg = ref<string>('')
const enableServer = useThrottleFn(async () => {
    loading.value = true
    invoke('local_shared_server', { path: path.value, stop: false })
        .then((res) => {
            msg.value = res as string
            loading.value = false
        })
        .catch((e) => {
            console.log(e)
            loading.value = false
            ElMessage.error('启用失败')
        })
}, 100)
</script>

<template>
    <div class="local-server">
        <el-input v-model="path">
            <template #prepend>
                <el-button @click="chooseDir">选择目录</el-button>
            </template>
            <template #append>
                <el-button @click="enableServer" :loading="loading">启用服务</el-button>
            </template>
        </el-input>
        <el-alert> {{ msg }} </el-alert>
    </div>
</template>

<style scoped lang="scss">
.color-conversion {
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>
