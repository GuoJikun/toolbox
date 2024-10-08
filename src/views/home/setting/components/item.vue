<script setup lang="ts">
interface Props {
    title: string
    description?: string
}
const props = withDefaults(defineProps<Props>(), {})
</script>

<template>
    <div class="setting-item">
        <p class="setting-item-title">{{ props.title }}</p>
        <p v-if="props.description" class="setting-item-description">{{ props.description }}</p>
        <div class="setting-item-body">
            <slot></slot>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.setting-item {
    color: var(--el-text-color-primary);
    & + & {
        margin-top: 12px;
    }

    &-title {
        font-size: 14px;
        font-weight: 500;
    }
    &-description {
        font-size: 12px;
        color: #666;
    }
    &-body {
        margin-top: 8px;
    }
}
</style>
