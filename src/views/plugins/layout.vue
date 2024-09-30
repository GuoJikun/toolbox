<script setup lang="ts">
import LayoutHeader from './layouts/header.vue';
</script>

<template>
    <div class="plugins__container">
        <LayoutHeader />
        <div>
            <router-view />
        </div>
    </div>
</template>

<style scoped lang="scss">
.plugins {
    &__container{
        padding: 24px;
    }
}
</style>