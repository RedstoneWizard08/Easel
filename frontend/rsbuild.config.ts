import { defineConfig } from "@rsbuild/core";
import { pluginVue } from "@rsbuild/plugin-vue";
import UnoCSS from "@unocss/postcss";

// Docs: https://rsbuild.rs/config/
export default defineConfig({
    plugins: [pluginVue()],

    html: {
        favicon: "src/assets/favicon.ico",
        title: "Loco SaaS Starter",
    },

    server: {
        port: 5151,
        strictPort: true,

        proxy: {
            "/api": {
                target: "http://127.0.0.1:5150",
                changeOrigin: true,
                secure: false,
            },
        },
    },

    tools: {
        postcss: {
            postcssOptions: {
                plugins: [UnoCSS()],
            },
        },
    },
});
