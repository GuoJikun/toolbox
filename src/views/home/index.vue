<script setup lang="ts">
import InstallPlugin from "@/components/install-plugin.vue";

const handlerInstallSuccess = (path: string) => {
  console.log("path", path);
};
</script>
<template>
  <div class="home">
    <h1>Home Index</h1>
    <router-link to="/search">Search</router-link>
    <div>
      <InstallPlugin tag="button" @confirm="handlerInstallSuccess">
        选择 目录
      </InstallPlugin>
    </div>
  </div>
</template>
