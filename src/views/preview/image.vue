not-support.vue<script setup lang="ts">
defineOptions({
    name: "ImageSupport",
})

interface Props {
    src?: string
}
const props = withDefaults(defineProps<Props>(),{
    src: ""
})
</script>

<template>
<div class="image-support">
    <img :src="props.src" alt="">
</div>
</template>

<style scoped lang="scss">
.image-support {
    width: 100%;
    height: 100%;
    & img {
        max-width: 100%;
        max-height: 100%;
    }
}
</style>