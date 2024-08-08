<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import LayouHeader from '@/layout/header.vue'

onMounted(async () => {
    console.log('home layout mounted')
})
</script>

<template>
    <div class="home-layout">
        <LayouHeader />
        <div class="home-layout-main">
            <div class="home-layout-aside">
                <router-link to="/setting">设置</router-link>
                <router-link to="/plugins">插件</router-link>
            </div>
            <div style="padding: 12px">
                <RouterView />
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.home-layout {
    background-color: antiquewhite;
    &-main {
        display: flex;
    }
    &-aside {
        flex: 0 0 40px;
        display: inline-flex;
        flex-direction: column;
    }
}
</style>
