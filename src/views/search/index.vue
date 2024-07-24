<script setup lang="ts">
import { ref } from 'vue'
import { formatPath } from '@/utils/index'
import { execBinaryPlugin, getPluginPath, getPluginOfPrefix, execScriptPlugin, execModulePlugin } from '@/utils/plugin'
import type { PluginConfig, InputFormater } from '@/utils/typescript'
import { useIndexStore } from '@/store'

import Search from './components/search.vue'
import Result from './components/result.vue'

const mainStore = useIndexStore()

const title = 'Search'

const keywords = ref<string>('')

interface Result {
    source: string
    command: string
    value: string
    raw?: PluginConfig | Record<string, unknown>
}
const resultList = ref<Array<Result>>([])

const parseInputContent = async (content: InputFormater) => {
    resultList.value = []
    const { prefix, value } = content
    if (value.length === 0) {
        return []
    }
    const keywords = value.map((c) => c.toLocaleLowerCase())
    console.log('keywords', keywords)
    if (prefix !== '') {
        console.log('有指定执行程序-即有前缀')

        const pluginConfig = getPluginOfPrefix(prefix, mainStore.plugins)
        if (!pluginConfig) {
            resultList.value.push({
                source: 'error',
                command: prefix,
                value: '未找到插件'
            })
            return
        }
        const pluginType = pluginConfig.type
        if (pluginType === 'binary') {
            const pluginPath = await getPluginPath(pluginConfig?.id)
            const binaryPath = await formatPath(pluginPath, 'toolbox-plugin-calc')
            const result = (await execBinaryPlugin(binaryPath, keywords)) as string
            resultList.value.push({
                source: 'binary',
                command: prefix,
                value: result,
                raw: pluginConfig
            })
        } else if (pluginType === 'script') {
            const { scriptEnv, main } = pluginConfig
            const pluginPath = await getPluginPath(pluginConfig?.id)
            const scriptPath = await formatPath(pluginPath, `/${main}`)
            const result = (await execScriptPlugin(scriptEnv, scriptPath, keywords)) as string
            resultList.value.push({
                source: 'script',
                command: prefix,
                value: result,
                raw: pluginConfig
            })
        }

        return
    } else {
        console.log('没有指定执行程序-即没有前缀')
        let keyword = keywords[0]
        resultList.value = [
            ...mainStore.plugins
                .filter((item) => {
                    if (item.type !== 'module') {
                        return false
                    }
                    if (
                        item.keywords?.some((c) => c.includes(keyword)) ||
                        item?.name.toLocaleLowerCase().includes(keyword)
                    ) {
                        return true
                    }
                    return false
                })
                .map((item) => {
                    console.log('item', 1)
                    return {
                        source: 'module',
                        command: '',
                        val: item.main,
                        value: item.description,
                        raw: item
                    }
                }),
            ...mainStore.installedPkg
                .filter((item) => item.name.toLocaleLowerCase().includes(keyword))
                .map((item) => {
                    return {
                        source: 'installedPkg',
                        command: '',
                        val: item.name,
                        value: item.name,
                        raw: item
                    }
                })
        ]
    }
}

const resultClick = async (item: any) => {
    console.log('item', item)
    if (item.type === 'module') {
        const pluginConfig = item.raw
        const { main } = pluginConfig
        // const pluginPath = await getPluginPath(pluginConfig?.id)
        // const indexPath = await formatPath(pluginPath, `/${main}`)
        const indexPath = `http://localhost:6543/${pluginConfig.id}/${main}`
        console.log('indexPath', indexPath)
        execModulePlugin(indexPath, pluginConfig)
    }
}
</script>
<template>
    <div class="search" data-tauri-drag-region>
        <h1>{{ title }}</h1>
        <Search v-model="keywords" @change="parseInputContent" />

        <Result :data="resultList" @click="resultClick" />
    </div>
</template>

<style lang="scss" scoped>
.search {
    padding: 24px;
    background-color: antiquewhite;
}
</style>
