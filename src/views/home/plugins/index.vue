<script setup lang="ts">
import { useRouter } from 'vue-router'
import { readDir, exists, readTextFile } from '@tauri-apps/plugin-fs'
import { getPluginsPath, getPluginPath, formatPath } from '@/utils'
import { ref, onMounted } from 'vue'

const router = useRouter()
const handleBack = () => {
    router.push('/')
}
interface Plugin {
    logo?: string | URL
    id: string
    name: string
    description: string
    version: string
}
const plugins = ref<Plugin[]>([])
const getPlugins = async () => {
    plugins.value = []
    const pluginsPath = await getPluginsPath()
    if (!(await exists(pluginsPath))) {
        plugins.value = []
        return []
    }
    const dirs = await readDir(pluginsPath)
    dirs.forEach(async (item) => {
        if (item.isDirectory) {
            const pluginPath = await getPluginPath(item.name)
            const configPath = await formatPath(pluginPath, '/config.json')
            const configObjString = await readTextFile(configPath)
            const config = JSON.parse(configObjString)

            plugins.value.push({
                id: config.id,
                name: config.name,
                description: config.description,
                version: config.version
            })
        }
    })
}

onMounted(() => {
    getPlugins()
})
</script>
<template>
    <div class="setting">
        <div class="setting-header">
            <div class="setting-haeder-back">
                <el-page-header @back="handleBack" content="插件">
                    <template #extra>
                        <div class="flex items-center">
                            <el-button size="small">安装本地插件</el-button>
                        </div>
                    </template>
                </el-page-header>
            </div>
            <h1 class="setting-haeder-title">插件</h1>
            <div>
                <div v-for="plugin in plugins" :key="plugin.id">
                    <div>
                        <div></div>
                        <div>
                            <p>{{ plugin.name }}-</p>
                            <p v-text="plugin.description"></p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.setting {
    &-header {
        position: relative;
        &-back {
            position: absolute;
        }
    }
}
</style>
