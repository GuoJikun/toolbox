<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView, useRoute, useRouter } from 'vue-router'
import LayouHeader from '@/layout/header.vue'
import { Config, System, Home } from '@icon-park/vue-next'
const route = useRoute()
const router = useRouter()
onMounted(async () => {
    console.log('home layout mounted')
    console.log('route', route.path)
    console.log('router', router.currentRoute.value.path)
})
</script>

<template>
    <div class="home-layout">
        <LayouHeader />
        <div class="home-layout-main">
            <div class="home-layout-aside">
                <router-link
                    to="/home"
                    :class="['home-layout-aside-item', { 'is-active': route.path === router.currentRoute.value.path }]"
                >
                    <Home theme="outline" size="20" />
                </router-link>
                <router-link
                    to="/plugins"
                    :class="['home-layout-aside-item', { 'is-active': route.path === router.currentRoute.value.path }]"
                >
                    <System theme="outline" size="20" />
                </router-link>
                <router-link
                    to="/setting"
                    :class="['home-layout-aside-item', { 'is-active': route.path === router.currentRoute.value.path }]"
                >
                    <Config theme="outline" size="20" />
                </router-link>
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
        padding: 16px 0;
        &-item {
            display: flex;
            justify-content: center;
            color: var(--el-text-color);
            & .is-active {
                color: var(--el-color-primary);
            }
            &:hover {
                color: var(--el-color-primary);
            }
            & + & {
                margin-top: 24px;
            }
        }
    }
}
</style>
