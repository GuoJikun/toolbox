<script setup lang="ts">
import { useRouter } from 'vue-router'
import SettingItem from './components/item.vue'

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
        <div>
            <SettingItem
                title="插件路径设置"
                description="设置插件的路径，如果没有配置则默认为 `$HOME/.vtools/plugins`"
            >
                <el-input readonly placeholder="点击选择插件目录">
                    <template #append>
                        <el-button>选择</el-button>
                    </template>
                </el-input>
            </SettingItem>
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
