<script setup lang="ts">
import InstallPlugin from '@/components/install-plugin.vue'

const handlerInstallSuccess = (path: string) => {
    console.log('path', path)
}
</script>
<template>
    <div class="home">
        <h1>Home Index</h1>
        <el-space>
            <router-link to="/setting">设置</router-link>
            <router-link to="/plugins">插件</router-link>
        </el-space>

        <div>
            <InstallPlugin tag="button" @confirm="handlerInstallSuccess"> 选择 目录 </InstallPlugin>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.home {
    padding: 12px;
}
</style>
