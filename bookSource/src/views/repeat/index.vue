<template>
  <div class="tab-container flex">
    <div class="tab-item">
      <a-button type="link" size="small" @click="handleChangeRouter('local')"
        >本地文件</a-button
      >
    </div>
    <div class="tab-item">
      <a-button type="link" size="small" @click="handleChangeRouter('online')"
        >网络文件</a-button
      >
    </div>
  </div>
  <div class="repeat-router-container">
    <router-view></router-view>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

// 切换路由
const handleChangeRouter = (name: string) => {
  router.push({ name });
};
</script>

<style scoped lang="scss">
.tab-container {
  padding-top: 5px;
  margin: 0 8px;
  border-bottom: 1px solid #ccc;
}
.repeat-router-container {
  height: calc(100% - 40px);
  margin-top: 2px;
  overflow-y: auto;
  overflow-x: hidden;
}
</style>
