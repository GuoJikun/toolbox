<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core'

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
}
const emit = defineEmits<Emits>()

let compositioned = false
const handleCompositionStart = () => {
    compositioned = true
}
const handleCompositionEnd = (e: Event) => {
    compositioned = false
    const target = e.target as HTMLInputElement
    emit('update:modelValue', target.value)
}
const handleInput = useDebounceFn((e: Event) => {
    const target = e.target as HTMLInputElement
    if (compositioned) {
        return
    }
    emit('update:modelValue', target.value)
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
