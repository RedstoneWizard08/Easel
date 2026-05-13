import { defineConfig, presetWind4 } from "unocss";

export default defineConfig({
    content: {
        filesystem: ["./src/**/*.{html,js,ts,jsx,tsx,vue}"],
    },

    theme: {
        font: {
            jbm: "JetBrains Mono",
            "jbm-v": "JetBrains Mono Variable",
            inter: "Inter",
            "inter-v": "Inter Variable",
        },
    },

    presets: [presetWind4()],
});
