import "./index.css";
import "@unocss/reset/tailwind-v4.css";
import { createApp } from "vue";
import { router } from "./lib/router";
import App from "./App.vue";

createApp(App).use(router).mount("#root");
