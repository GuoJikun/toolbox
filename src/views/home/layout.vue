<script setup lang="ts">
import { onMounted } from "vue";
import { RouterView } from "vue-router";
import LayouHeader from "@/layout/header.vue";

import { Menu, MenuItem, Submenu } from "@tauri-apps/api/menu";
import { getCurrent } from "@tauri-apps/api/window";

const addMenu = async () => {
  const menu = await Menu.new();
  console.log("menu", menu);
  const about = await Submenu.new({
    text: "帮助",
  });
  about.append(
    await MenuItem.new({
      text: "关于",
      action: () => {
        console.log("click about");
      },
    })
  );
  console.log("about", about);
  menu.append([about]);

  const curWindow = await getCurrent();
  menu.setAsWindowMenu(curWindow);
};

onMounted(async () => {
  console.log("home layout mounted");
  addMenu();
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
