import { RouteRecordRaw } from "vue-router";

export const routes: RouteRecordRaw[] = [
  {
    // ANCHOR - Comment this out to be redirected to dashboard
    path: "/",
    name: "Overview",
    component: () => import("@/App.vue"),
  },
  {
    path: "/settings",
    name: "Settings",
    component: () => import("@/pages/Settings.vue"),
  },
];
