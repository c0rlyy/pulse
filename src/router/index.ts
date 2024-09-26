import { createRouter, createWebHistory } from "vue-router";
import WelcomeView from "../views/WelcomeView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "welcome",
      component: WelcomeView,
      children: [
        {
          path: "/login",
          name: "login",
          component: () => import("../components/UI/LoginOption.vue"),
        },
        {
          path: "/register",
          name: "register",
          component: () => import("../components/UI/RegisterOption.vue"),
        },
        {
          path: "/anonymous",
          name: "anonymous",
          component: () => import("../components/UI/AnonymousOption.vue"),
        },
      ],
    },
    {
      path: "/home",
      name: "home",
      component: () => import("../views/HomeView.vue"),
    },
  ],
});

export default router;
