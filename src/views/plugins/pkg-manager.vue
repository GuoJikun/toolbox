<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'

interface Item {
    Name: string
    Id: string
    InstalledVersion: string
    IsUpdateAvailable: boolean
    Source: string
    AvailableVersions: Array<string>
}

type List = Array<Item>
const list = ref<List>([])

const parseStr2List = (str: string) => {
    return JSON.parse(str) as List
}

const loading = ref<boolean>(false)
const fetchList = async () => {
    loading.value = true
    invoke('installed_list')
        .then((res) => {
            list.value = parseStr2List(res as string)
            console.log(list.value)
        })
        .catch((e) => {
            console.log(e)
            ElMessage.error('获取列表失败')
        })
        .finally(() => {
            loading.value = false
        })
}

const checkUpdate = async (name: string) => {
    invoke('check_updates', { name })
        .then((res) => {
            console.log(res)
        })
        .catch((e) => {
            console.log(e)
            ElMessage.error('检查更新失败')
        })
}
</script>

<template>
    <div class="local-server">
        <div style="display: flex; justify-content: space-between">
            <div>软件管理</div>
            <div>
                <el-button type="primary" @click="fetchList">刷新</el-button>
                <el-button type="primary" @click="checkUpdate">检查更新</el-button>
            </div>
        </div>
        <table layout="fixed" style="width: 100%" v-loading="loading">
            <colgroup>
                <col style="width: 140px" />
                <col />
                <col style="width: 60px" />
                <col style="width: 60px" />
            </colgroup>
            <thead>
                <tr class="row">
                    <th class="row-item">名称</th>
                    <th class="row-item">ID</th>
                    <th class="row-item">版本</th>
                    <th class="row-item">来源</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="item in list" :key="item.Id" class="row">
                    <td class="row-item">{{ item.Name }}</td>
                    <td class="row-item">{{ item.Id }}</td>
                    <td class="row-item" style="flex: 0 0 60px">{{ item.InstalledVersion }}</td>
                    <td class="row-item" style="flex: 0 0 60px">{{ item.Source }}</td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<style scoped lang="scss">
.color-conversion {
    display: flex;
    justify-content: center;
    align-items: center;
}
.row {
    &-item {
        flex: 1 0 140px;
        padding: 4px 8px;
        border: 1px solid #ccc;
        text-align: center;
    }
}
</style>
