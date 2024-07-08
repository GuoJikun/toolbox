<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
const handleBack = () => {
    router.push('/')
}
</script>
<template>
    <div class="setting">
        <div class="setting-header">
            <div class="setting-haeder-back">
                <el-page-header @back="handleBack"></el-page-header>
            </div>
            <h1 class="setting-haeder-title">设置</h1>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.setting {
    &-header {
        position: relative;
        &-back {
            position: absolute;
        }
    }
}
</style>
