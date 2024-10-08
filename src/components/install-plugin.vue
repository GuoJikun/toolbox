<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

interface InstallPluginProps {
  tag: string;
  dialogTitle?: string;
}
const props = withDefaults(defineProps<InstallPluginProps>(), {
  tag: "div",
  dialogTitle: "选择文件夹",
});

type InstallPluginEvents = {
  confirm: [val: string];
  cancel: [];
  error: [err: Error];
};

const emit = defineEmits<InstallPluginEvents>();
const handlerClick = () => {
  open({
    directory: true,
    multiple: false,
    title: props.dialogTitle,
  })
    .then((path) => {
      if (path) {
        emit("confirm", path);
      } else {
        emit("cancel");
      }
    })
    .catch((err) => {
      emit("error", err);
    });
};
</script>

<template>
  <component :is="props.tag" @click="handlerClick">
    <slot></slot>
  </component>
</template>
