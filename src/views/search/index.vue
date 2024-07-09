<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core'

const title = 'Search'

const parseInputContent = (content: string) => {
    console.log(content)
    const input = content.trim()
    if (input === '') {
        return
    }
    console.log(input)
    const args = input.split(' ')
    if (args.length > 0) {
        const command = args[0]
        console.log(command)
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
</style>
