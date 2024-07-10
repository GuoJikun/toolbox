<script setup lang="ts">
import { ref } from 'vue'
import { useDebounceFn } from '@vueuse/core'
import { formatPath } from '@/utils/index'
import { execBinaryPlugin, getPluginPath, getPluginOfPrefix } from '@/utils/plugin'
import { useIndexStore } from '@/store'

const mainStore = useIndexStore()

const title = 'Search'

const resultList = ref<Array<Record<string, unknown>>>([])

const parseInputContent = async (content: string) => {
    const input = content.trim()
    if (input === '') {
        return
    }
    const args = input.split(' ')
    console.log('args', args)
    if (args.length > 1) {
        const [command, val] = args

        const pluginConfig = getPluginOfPrefix(command, mainStore.plugins)
        if (!pluginConfig) {
            resultList.value.push({
                command,
                val,
                value: '未找到插件'
            })
            return
        }
        const pluginPath = await getPluginPath(pluginConfig?.id)
        const binaryPath = await formatPath(pluginPath, 'toolbox-plugin-calc')
        const result = await execBinaryPlugin(binaryPath, [val])
        resultList.value.push({
            command,
            val,
            value: result
        })
        return
    }
}

let compositioned = false
const handleCompositionStart = () => {
    compositioned = true
}
const handleCompositionEnd = (e: Event) => {
    compositioned = false
    const target = e.target as HTMLInputElement
    parseInputContent(target.value)
}
const handleInput = useDebounceFn((e: Event) => {
    resultList.value = []
    const target = e.target as HTMLInputElement
    if (compositioned) {
        return
    }
    parseInputContent(target.value)
}, 200)
</script>
<template>
    <div class="search" data-tauri-drag-region>
        <h1>{{ title }}</h1>
        <div class="input">
            <input
                type="text"
                class="input-inner"
                @input="handleInput"
                @compositionstart="handleCompositionStart"
                @compositionend="handleCompositionEnd"
            />
        </div>
        <div class="result">
            <div v-for="(item, index) in resultList" :key="index" class="result-item">
                <p>{{ item.command }}</p>
                <p>{{ item.val }}</p>
                <p>{{ item.value }}</p>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.search {
    padding: 24px;
    background-color: antiquewhite;
}
.input {
    border-radius: 8px;
    overflow: hidden;
    &-inner {
        border-radius: 8px;
        display: block;
        border: 1px solid var(--el-border-color);
        outline: none;
        height: 48px;
        width: 100%;
        padding: 0 16px;
        font-size: 16px;
        color: var(--el-text-color-primary);
        box-shadow: inset 0 0 10px 0 rgba(51, 51, 51, 0.14);
    }
}

.result {
    margin-top: 12px;
    &-item {
        padding: 6px 12px;
        font-size: 14px;
    }
}
</style>
