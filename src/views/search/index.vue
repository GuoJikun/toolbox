<script setup lang="ts">
import { ref } from 'vue'
import { formatPath } from '@/utils/index'
import { execBinaryPlugin, getPluginPath, getPluginOfPrefix, execScriptPlugin, execModulePlugin } from '@/utils/plugin'
import { useIndexStore } from '@/store'

import Search from './components/search.vue'
import Result from './components/result.vue'

const mainStore = useIndexStore()

const title = 'Search'

const keywords = ref<string>('')
const resultList = ref<Array<Record<string, unknown>>>([])

const parseInputContent = async (content: string) => {
    resultList.value = []
    const input = content.trim()
    if (input === '') {
        return
    }
    const args = input.split(' ')
    console.log('args', args)
    if (args.length > 1) {
        const [command, ...val] = args

        const pluginConfig = getPluginOfPrefix(command, mainStore.plugins)
        if (!pluginConfig) {
            resultList.value.push({
                type: 'error',
                command,
                val,
                value: '未找到插件'
            })
            return
        }
        const pluginType = pluginConfig.type
        if (pluginType === 'binary') {
            const pluginPath = await getPluginPath(pluginConfig?.id)
            const binaryPath = await formatPath(pluginPath, 'toolbox-plugin-calc')
            const result = await execBinaryPlugin(binaryPath, val)
            resultList.value.push({
                type: 'result',
                command,
                val,
                value: result,
                raw: pluginConfig
            })
        } else if (pluginType === 'script') {
            const { scriptEnv, main } = pluginConfig
            const pluginPath = await getPluginPath(pluginConfig?.id)
            const scriptPath = await formatPath(pluginPath, `/${main}`)
            const result = await execScriptPlugin(scriptEnv, scriptPath, val || [])
            resultList.value.push({
                type: 'result',
                command,
                val,
                value: result,
                raw: pluginConfig
            })
        }

        return
    } else {
        console.log('插件类型是 Module')
        resultList.value = mainStore.plugins
            .filter((item) => {
                if (item.type !== 'module') {
                    return false
                }
                if (item.keywords?.some((c) => c.includes(input)) || item?.name.includes(input)) {
                    return true
                }
                return false
            })
            .map((item) => {
                console.log('item', 1)
                return {
                    type: 'module',
                    command: '',
                    val: item.main,
                    value: item.description,
                    raw: item
                }
            })
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
        <Search v-model="keywords" @update:modelValue="parseInputContent" />

        <Result :data="resultList" @click="resultClick" />
    </div>
</template>

<style lang="scss" scoped>
.search {
    padding: 24px;
    background-color: antiquewhite;
}
</style>
