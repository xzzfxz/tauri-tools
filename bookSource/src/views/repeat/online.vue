<template>
  <div class="search-container flex">
    <div class="search-item" style="width: 100%">
      <a-input
        v-model:value="onlineUrl"
        placeholder="请输入在线地址"
        style="width: 80%"
        allowClear
      ></a-input>
    </div>
  </div>
  <div class="search-container flex">
    <div class="search-item">
      <a-button type="primary" @click="handleDeleteRepeat" :loading="loading"
        >开始去重</a-button
      >
    </div>
    <div class="search-item">
      <a-button
        type="primary"
        :disabled="!hasDeleteRepeated"
        @click="handleDownload"
        >下载</a-button
      >
    </div>
  </div>
</template>

<script setup lang="ts">
import { save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { message, notification } from "ant-design-vue";

const loading = ref(false);
const hasDeleteRepeated = ref(false);
const onlineUrl = ref("");

const isJsonUrl = (url: string) => {
  // 使用正则表达式检查链接格式和以 .json 结尾
  const regex =
    /^(https?:\/\/)?([\w-]+\.)*[\w-]+(\.[a-z]{2,})+(\/\S*)?\.json$/i;
  return regex.test(url);
};

// 开始去重
const handleDeleteRepeat = () => {
  if (!isJsonUrl(onlineUrl.value)) {
    message.warning("链接不符合规范");
    return;
  }
  loading.value = true;
  invoke("delete_online_repeat", { url: onlineUrl.value })
    .then((res: any) => {
      if (res.code === 0) {
        hasDeleteRepeated.value = true;
        const result = res.result || {};
        notification.success({
          message: "去重成功",
          description: `去重前：${result.preLen}条，去重后：${result.curLen}条`,
        });
      } else {
        notification.error({
          message: "去重失败",
          description: res.msg,
        });
      }
    })
    .finally(() => {
      loading.value = false;
    });
};

// 下载
const handleDownload = () => {
  save({
    defaultPath: "result.json",
    filters: [
      {
        name: "Json",
        extensions: ["json"],
      },
    ],
  }).then((res: any) => {
    if (!res) {
      return;
    }
    invoke("download_file", { savePath: res }).then((res: any) => {
      if (res.code === 0) {
        message.success("下载成功");
      } else {
        message.error(res.msg);
      }
    });
  });
};
</script>

<style scoped lang="scss">
.search-container {
  padding-top: 10px;
  margin-left: 10px;
}
</style>
