import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";
import sveltePreprocess from "svelte-preprocess";

// https://vitejs.dev/config/
export default defineConfig({
    server: {
        proxy: {
            "^/(api|auth)": {
                target: "http://127.0.0.1:8080",
            },
        },
    },
    plugins: [
        svelte({
            preprocess: [
                sveltePreprocess({
                    includePaths: ["src/styles"],
                }),
            ],
        }),
    ],
    resolve: {
        alias: {
            $static: resolve("./static"),
            $assets: resolve("./static/assets"),
            $fonts: resolve("./static/assets/fonts"),
        },
    },
    build: {
        watch: {
            include: ["../src/**/*.scss"],
        },
    },
});
