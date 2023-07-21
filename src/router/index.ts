import { createRouter, createWebHashHistory } from "vue-router";
// import type { RouteRecordRaw } from "vue-router";

import Register from "../views/register/index.vue";
import Login from "../views/login/index.vue";
import Home from "../views/home/index.vue";
import File from "../views/file/index.vue";

const routes = [
  {
    path: "/",
    name: "Login",
    component: Login,
  },
  {
    path: "/register",
    name: "Register",
    component: Register,
  },
  {
    path: "/home",
    name: "Home",
    component: Home,
  },
  {
    path: "/file",
    name: "File",
    component: File,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

router.beforeEach((to, from) => { });

router.afterEach(() => { });

export default router;
