<script lang="ts">
    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "@skeletonlabs/skeleton/styles/skeleton.css";
    import "../app.postcss";
    import "../themes/base.postcss";
    import "../themes/colors.postcss";

    import hljs from "highlight.js";
    import { storeHighlightJs } from "@skeletonlabs/skeleton";
    import "highlight.js/styles/agate.css";
    storeHighlightJs.set(hljs);

    import { Toast } from "@skeletonlabs/skeleton";

    import {
        computePosition,
        autoUpdate,
        offset,
        shift,
        flip,
        arrow,
    } from "@floating-ui/dom";
    import { storePopup } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";
    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    let init = false;
    onMount(() => (init = true));
</script>

<Toast />

<slot />
