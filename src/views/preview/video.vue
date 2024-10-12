<script setup lang="ts">
defineOptions({
    name: "VideoSupport",
})

interface Props {
    src?: string
}
const props = withDefaults(defineProps<Props>(),{
    src: ""
})
</script>

<template>
    <div class="video-support">
        <video :src="props.src" controls></video>
    </div>
</template>

<style scoped lang="scss">
.video-support {
    width: 100%;
    height: 100%;
    & video {
        max-width: 100%;
        max-height: 100%;
    }
}
</style>