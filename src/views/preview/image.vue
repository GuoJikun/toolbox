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
    <div>
        <img :src="props.src" alt="">
    </div>
</div>
</template>

<style scoped lang="scss">
.image-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    & img {
        max-width: 100%;
        max-height: 100%;
    }
}
</style>