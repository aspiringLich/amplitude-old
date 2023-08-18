import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { purgeCss } from "vite-plugin-tailwind-purgecss";
// import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "path";

export default defineConfig({
    server: {
        proxy: {
            "^/(api|auth)": {
                target: "http://127.0.0.1:8080",
            },
        },
    },
    plugins: [sveltekit(), purgeCss()],
    resolve: {
        alias: {
            $src: path.resolve("./src"),
            $lib: path.resolve("./src/lib"),
            $cmpt: path.resolve("./src/components"),
            $styles: path.resolve("./src/styles"),
            $static: path.resolve(`./static`),
            $assets: path.resolve(`./static/assets`),
        },
    },
});
