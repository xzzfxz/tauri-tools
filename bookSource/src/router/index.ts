import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    redirect: "/repeat",
  },
  {
    path: "/repeat",
    component: () => import("@/views/repeat/index.vue"),
    redirect: "/repeat/local",
    children: [
      {
        path: "local",
        name: "local",
        component: () => import("@/views/repeat/local.vue"),
      },
      {
        path: "online",
        name: "online",
        component: () => import("@/views/repeat/online.vue"),
      },
    ],
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
