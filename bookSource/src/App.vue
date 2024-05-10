<template>
  <div class="body-container">
    <router-view></router-view>
  </div>
  <Update />
</template>

<script setup lang="ts">
import Update from "@/components/Update.vue";
</script>

<style scoped lang="scss">
.body-container {
  width: 100vw;
  height: 100vh;
}
</style>
