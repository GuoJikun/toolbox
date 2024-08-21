<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { formatPath, runSoftware, getWindow } from '@/utils/index'
import { execBinaryPlugin, getPluginPath, getPluginOfPrefix, execScriptPlugin, execModulePlugin } from '@/utils/plugin'
import type { PluginConfig, InputFormater } from '@/utils/typescript'
import { useIndexStore } from '@/store'

import Search from './components/search.vue'
import Result from './components/result.vue'

const mainStore = useIndexStore()

const inputEl = ref<HTMLInputElement>()

onMounted(async () => {
    console.log('search mounted')
    // 加载完成时给 Window 绑定事件
    const searchWindow = await getWindow('search')
    console.log('searchWindow', searchWindow)
    searchWindow?.once('tauri://blur', () => {
        searchWindow.hide()
    })

    searchWindow?.listen('tauri://focus', () => {
        inputEl.value?.focus()
    })
})

const keywords = ref<string>('')

interface Result {
    source: string
    name: string
    value: string
    raw?: PluginConfig | Record<string, unknown>
}
const resultList = ref<Array<Result>>([])

const parseInputContent = async (content: InputFormater) => {
    console.log('content', content)
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
                name: prefix,
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
                name: prefix,
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
                name: prefix,
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
                        name: item.main,
                        value: item.description,
                        raw: item
                    }
                }),
            ...mainStore.installedPkg
                .filter((item) => item.name.toLocaleLowerCase().includes(keyword))
                .map((item) => {
                    return {
                        source: 'installedPkg',
                        name: item.name,
                        value: item.path,
                        raw: item
                    }
                })
        ]
    }
}

const resultClick = async (item: any) => {
    console.log('item', item)
    if (!['module', 'installedPkg'].includes(item.source)) {
        return
    }
    if (item.source === 'module') {
        const pluginConfig = item.raw
        const { main } = pluginConfig

        let indexPath = `http://localhost:6543/${pluginConfig.id}/${main}`
        if (pluginConfig.id === 'screenshot') {
            indexPath = pluginConfig.devMain
        }
        console.log('indexPath', indexPath)
        execModulePlugin(indexPath, pluginConfig)
    } else if (item.source === 'installedPkg') {
        console.log('执行已安装的程序包')
        if (item.raw.path) {
            runSoftware(item.raw.path)
        } else {
            console.log('未找到可执行文件')
        }
    }
    const searchWindow = await getWindow('search')
    searchWindow?.hide()
    keywords.value = ''
    resultList.value = []
}
</script>
<template>
    <div class="search" data-tauri-drag-region>
        <Search ref="inputEl" v-model="keywords" @change="parseInputContent" />

        <Result :data="resultList" @click="resultClick" />
    </div>
</template>

<style lang="scss" scoped>
.search {
    padding: 24px;
    background-color: antiquewhite;
}
</style>
