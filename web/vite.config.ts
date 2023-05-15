import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import path from "path";

export default defineConfig({
    server: {
        proxy: {
            "^/(api|auth)": {
                target: "http://127.0.0.1:8080",
            },
        },
    },
    plugins: [sveltekit()],
    resolve: {
        alias: {
            "@src": path.resolve("./src"),
            "@styles": path.resolve("./src/styles"),
            "@static": path.resolve("./"),
            "@assets": path.resolve("./assets"),
        },
    },
});
