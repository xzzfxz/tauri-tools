import { createRouter, createWebHistory } from "vue-router";
import Search from "../views/search/index.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "search",
      component: Search,
    },
    {
      path: "/detail",
      name: "detail",
      component: () => import("../views/detail/index.vue"),
    },
    {
      path: "/content",
      name: "content",
      component: () => import("../views/content/index.vue"),
    },
  ],
});

export default router;
