<script setup lang="ts">
import LayoutHeader from './layouts/header.vue';
</script>

<template>
    <div>
        <LayoutHeader />
        <div>
            <router-view />
        </div>
    </div>
</template>

<style scoped lang="scss">

</style>