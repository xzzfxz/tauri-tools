<template>
  <div class="header-container">
    <button @click="handleSearch">搜索</button>
  </div>
  <div class="content-container">
    <div>返回结果：</div>
    <div>{{ result }}</div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const result = ref("");

const handleSearch = () => {
  invoke("search", {
    text: "我的",
  }).then((res: any) => {
    result.value = res;
  });
};
</script>

<style scoped>
.content-container {
  margin-top: 10px;
}
</style>
