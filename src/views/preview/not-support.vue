<script setup lang="ts">
defineOptions({
    name: "NotSupport",
})

interface Props {
    src: string
}
const props = withDefaults(defineProps<Props>(),{
    src: ""
})
</script>

<template>
<div>暂不支持此格式的预览</div>
</template>

<style scoped lang="scss">

</style>