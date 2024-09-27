<template>
  <div class="header-container">
    <button @click="handleSearch">搜索</button>
  </div>
  <div class="content-container">
    <div>返回结果：</div>
    <div
      v-for="(item, index) in result"
      :key="item.book_name"
      class="line-item"
    >
      <div class="title">{{ index + 1 + ". " + item.book_name }}</div>
      <div class="author">{{ item.author }}</div>
      <div class="abstract">{{ item.book_abstract }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const result = ref<any>([]);

const handleSearch = () => {
  invoke("search", {
    text: "我的",
  }).then((res: any) => {
    if (res.code === 0) {
      result.value = res.result;
    }
  });
};
</script>

<style scoped>
.content-container {
  margin-top: 10px;
}
.line-item {
  margin-top: 10px;
}
</style>
