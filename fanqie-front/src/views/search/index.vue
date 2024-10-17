<template>
  <div class="search-container flex">
    <div class="search-input">
      <a-input v-model:value="keyword" />
    </div>
    <div class="search-input">
      <a-button :loading="loading" @click="handleSearch">搜索</a-button>
    </div>
  </div>
  <div class="list-container">
    <div class="list-item flex" v-for="item in dataList" :key="item.book_id">
      <div class="image">
        <img :src="item.thumb_url" />
      </div>
      <div class="right-content">
        <div class="title click-active" @click="handleToDetail(item)">
          {{ item.book_name }}
        </div>
        <div class="author">{{ item.author }}</div>
        <div class="description">{{ item.book_abstract }}</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { fetchSearch } from "@/service";
import { ref } from "vue";
import { useRouter } from "vue-router";

const keyword = ref("我的");
const loading = ref(false);
const dataList = ref<any[]>([]);

// 搜索
const handleSearch = () => {
  loading.value = true;
  fetchSearch({ text: keyword.value })
    .then((res: any) => {
      dataList.value = res || [];
    })
    .finally(() => {
      loading.value = false;
    });
};

// 跳转到详情页
const handleToDetail = (current: any) => {
  const router = useRouter();
  router.push({ path: "/detail", query: { id: current.book_id } });
};
</script>

<style lang="scss" scoped>
.search-container {
  margin: 10px;
}
.list-container {
  height: calc(100vh - 70px);
  margin-top: 10px;
  overflow-y: auto;
}
.list-item {
  margin: 0 10px 10px;
  .right-content {
    margin-left: 10px;
  }
  .title {
    font-size: 18px;
    font-weight: bold;
    cursor: pointer;
  }
  .description {
    font-size: 12px;
    color: gray;
  }
}
.image {
  width: 80px;
  flex-shrink: 0;
  img {
    width: 100%;
  }
}
</style>
