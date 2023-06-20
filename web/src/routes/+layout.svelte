<script lang="ts">
    //~ Themes
    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "@skeletonlabs/skeleton/styles/skeleton.css";
    import "../app.postcss";
    import "../themes/base.postcss";
    import "../themes/colors.postcss";

    //~ `highlight.js`
    import hljs from "highlight.js";
    import { storeHighlightJs } from "@skeletonlabs/skeleton";
    import "highlight.js/styles/agate.css";
    storeHighlightJs.set(hljs);

    //~ Toast Notifications
    import { Toast } from "@skeletonlabs/skeleton";

    //~ Popups
    import {
        computePosition,
        autoUpdate,
        offset,
        shift,
        flip,
        arrow,
    } from "@floating-ui/dom";
    import { storePopup } from "@skeletonlabs/skeleton";
    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    //~ Modals
    import { Modal } from "@skeletonlabs/skeleton";
    import type { ModalComponent } from "@skeletonlabs/skeleton";
    import EditorSettings from "$cmpt/EditorSettings.svelte";
    const modalComponentRegistry: Record<string, ModalComponent> = {
        // Custom Modal 1
        EditorSettings: {
            ref: EditorSettings,
        },
    };
</script>

<Toast />
<Modal components={modalComponentRegistry} />

<slot />
