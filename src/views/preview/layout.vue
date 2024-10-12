<script setup lang="ts">
import { ref, shallowRef, type ComponentInstance } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { extname } from '@tauri-apps/api/path';
import { getWindow  } from "@/utils/window"
import Header from './components/header.vue'
import Footer from './components/footer.vue'
import NotSupport from './not-support.vue'
import ImageSupport from './image.vue'
import VideoSupport from './video.vue'

const path = ref<string>('')
let componentName = shallowRef<ComponentInstance<any>>(NotSupport)

const imgExtensions = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp']
const isImage = (path: string) => {
  return imgExtensions.some(ext => path.endsWith(ext))
}
const videoExtensions = ['mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv']
const isVideo = (path: string) => {
  return videoExtensions.some(ext => path.endsWith(ext))
}

const init = async () => {
  const win = await getWindow('preview')
  win?.listen('file-preview', async (e) => {

      const payload = e.payload as string
      const localePath = convertFileSrc(payload);
      console.log(localePath)
      const ext = await extname(payload);
      if (isImage(ext)) {
          componentName.value = ImageSupport
      }else if (isVideo(ext)){
          componentName.value = VideoSupport
      }else{
          componentName.value = NotSupport
      }
      path.value = localePath;
  })
}

init()

</script>

<template>
    <div class="preview">
        <Header class="preview-header" />
        <div class="preview-body">
            <component :is="componentName" :src="path"></component>
        </div>
        <Footer class="preview-footer" />
    </div>
</template>

<style scoped lang="scss">
.preview {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    position: relative;
    &-header{
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
    }
    &-footer{
        position: absolute;
        left: 0;
        bottom: 0;
        width: 100%;
        font-size: 12px;
    }
    &-body {
        padding: 28px 0 20px;
        display: flex;
        justify-content: center;
        align-items: center;
        align-content: center;
    }
}
</style>