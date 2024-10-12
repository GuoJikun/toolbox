<script setup lang="ts">
import { ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { getWindow  } from "@/utils/window"

const path = ref<string>('')

const init = async () => {
  const win = await getWindow('preview')
  win?.listen('file-preview', (e) => {
      console.log(e)
      const payload = e.payload as string
      const localePath = convertFileSrc(payload);
      console.log(localePath)
      path.value = localePath;
  })
}

init()

</script>

<template>
    <div>文件预览</div>
    <img :src="path" alt="" />
</template>

<style scoped lang="scss">

</style>