<script lang="ts">
    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "@skeletonlabs/skeleton/styles/skeleton.css";
    import "../app.postcss";
    import "../themes/base.postcss";
    import "../themes/colors.postcss";

    import hljs from "highlight.js";
    import { Toast, storeHighlightJs } from "@skeletonlabs/skeleton";
    import "highlight.js/styles/agate.css";

    storeHighlightJs.set(hljs);

    import {
        computePosition,
        autoUpdate,
        offset,
        shift,
        flip,
        arrow,
    } from "@floating-ui/dom";
    import { storePopup } from "@skeletonlabs/skeleton";
    import { browser } from "$app/environment";
    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
</script>

<slot />
<Toast />
