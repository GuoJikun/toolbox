<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core'
import type { InputFormater } from '@/utils/typescript'

interface Props {
    modelValue?: string
    delay?: number
}

const props = withDefaults(defineProps<Props>(), {
    modelValue: '',
    delay: 260
})

type Emits = {
    'update:modelValue': [value: string]
    change: [value: InputFormater]
}
const emit = defineEmits<Emits>()

const parseInput = (content: string): InputFormater => {
    const input = content.trim()
    console.log(input)
    if (input === '') {
        return {
            prefix: '',
            value: []
        }
    }
    const args = input.split(' ')
    if (args.length > 1) {
        const [command, ...val] = args
        return {
            prefix: command,
            value: val
        }
    } else {
        return {
            prefix: '',
            value: args
        }
    }
}

let compositioned = false
const handleCompositionStart = () => {
    compositioned = true
}
const handleCompositionEnd = (e: Event) => {
    compositioned = false
    const target = e.target as HTMLInputElement
    emit('update:modelValue', target.value)
    emit('change', parseInput(target.value))
}
const handleInput = useDebounceFn((e: Event) => {
    const target = e.target as HTMLInputElement
    if (compositioned) {
        return
    }

    emit('update:modelValue', target.value)
    emit('change', parseInput(target.value))
}, props.delay)
</script>
<template>
    <div class="input">
        <input
            type="text"
            class="input-inner"
            @input="handleInput"
            @compositionstart="handleCompositionStart"
            @compositionend="handleCompositionEnd"
        />
    </div>
</template>

<style lang="scss" scoped>
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
