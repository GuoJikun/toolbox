<script setup lang="ts">
import { onMounted } from "vue";
import { RouterView } from "vue-router";
import LayouHeader from "@/layout/header.vue";

onMounted(async () => {
  console.log("home layout mounted");
});
</script>

<template>
  <div class="home-layout">
    <LayouHeader />
    <RouterView />
  </div>
</template>

<style lang="scss" scoped>
.home-layout {
  background-color: antiquewhite;
}
</style>
