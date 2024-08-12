<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core'
import { ElDivider } from 'element-plus'
import { h, PropType } from 'vue'

const spacer = h(ElDivider, { style: { margin: '0' } })

interface Props {
    data: any[]
}
const props = defineProps({
    data: {
        type: Array as PropType<Props['data']>,
        required: true
    }
})

type Emits = {
    click: [item: Record<string, unknown>]
}

const emit = defineEmits<Emits>()
const handleClick = useDebounceFn((item: Record<string, unknown>) => {
    emit('click', item)
}, 100)
</script>

<template>
    <el-space direction="vertical" :spacer="spacer" fill class="result">
        <div v-for="(item, index) in props.data" :key="index" class="result-item" @click.stop="handleClick(item)">
            <p>{{ item.name }}</p>
            <p>{{ item.value }}</p>
        </div>
    </el-space>
</template>

<style lang="scss" scoped>
.result {
    width: 100%;
    margin-top: 12px;
    &-item {
        padding: 6px 12px;
        font-size: 14px;
        cursor: pointer;
        &:hover {
            background-color: #f0f0f0;
        }
    }
}
</style>
