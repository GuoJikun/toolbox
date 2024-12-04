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
        <div>
            <video :src="props.src" controls></video>
        </div>
    </div>
</template>

<style scoped lang="scss">
.video-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    & video {
        max-width: 100%;
        max-height: 100%;
    }
}
</style>