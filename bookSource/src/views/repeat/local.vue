<template>
  <div class="search-container flex">
    <div class="search-item">
      <a-button type="primary" @click="handleShowDialog">上传文件</a-button>
    </div>
    <div class="search-item">
      <a-button danger @click="handleClear">清空</a-button>
    </div>
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
  <div class="file-container">
    <a-spin :spinning="loading" wrapperClassName="spin-wrapper">
      <div class="file-item" v-for="(item, index) in filePathList" :key="item">
        {{ index + 1 + "：" + item }}
      </div>
    </a-spin>
  </div>
</template>

<script setup lang="ts">
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { message, notification } from "ant-design-vue";

const filePathList = ref<string[]>([]);
const loading = ref(false);
const hasDeleteRepeated = ref(false);

// 显示文件选择框
const handleShowDialog = () => {
  open({
    multiple: true, // 是否可以选择多个文件
    filters: [
      {
        name: "Json",
        extensions: ["json"],
      },
    ],
  }).then((files: any) => {
    if (!files) {
      // 取消选择
      return;
    }
    const set = new Set([...filePathList.value, ...files]);
    filePathList.value = Array.from(set);
  });
};

// 清空选择
const handleClear = () => {
  filePathList.value = [];
  hasDeleteRepeated.value = false;
};

// 开始去重
const handleDeleteRepeat = () => {
  if (!filePathList.value.length) {
    message.warning("请先上传文件");
    return;
  }
  loading.value = true;
  invoke("delete_repeat", { pathList: filePathList.value })
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
.tab-container {
  margin-top: 5px;
  margin-left: 4px;
}
.search-container {
  padding-top: 10px;
  margin-left: 10px;
}
.file-container {
  height: calc(100% - 70px);
  margin: 0 10px 0;
  padding: 0 4px;
  border: 1px solid #ccc;
  border-radius: 4px;
  overflow-y: auto;
  font-size: 14px;
  &::-webkit-scrollbar {
    width: 6px;
  }
}
.spin-wrapper {
  width: 100%;
  padding-top: 10px;
}
</style>
