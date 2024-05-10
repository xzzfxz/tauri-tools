<template>
  <a-modal
    v-model:open="showModal"
    title="检查更新"
    @ok="handleOk"
    @cancel="handleCancel"
  >
    <div>检测到新版本，是否更新？</div>
  </a-modal>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";

const emit = defineEmits(["cancel", "ok"]);

const showModal = ref(false);

// 检查更新
const checkIsUpdate = async () => {
  try {
    const { shouldUpdate } = await checkUpdate();
    if (shouldUpdate) {
      showModal.value = true;
    }
  } catch (e) {
    console.log("======", e);
  }
};

const handleOk = async () => {
  // 安装更新
  await installUpdate();
  // 重新启动应用
  await relaunch();
  showModal.value = false;
};

const handleCancel = () => {
  emit("cancel");
};

checkIsUpdate();
</script>

<style lang="scss" scoped></style>
