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
            "@styles": path.resolve(__dirname, "src/styles"),
        },
    },
});
