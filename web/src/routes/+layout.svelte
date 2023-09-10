<script lang="ts">
    //~ Themes
    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "../app.postcss";

    //~ `highlight.js`
    import hljs from "highlight.js";
    import { Drawer, storeHighlightJs } from "@skeletonlabs/skeleton";
    import "highlight.js/styles/agate.css";
    storeHighlightJs.set(hljs);

    //~ Init Stores
    import { initializeStores, getDrawerStore, getToastStore } from "@skeletonlabs/skeleton";
    initializeStores();
    const drawerStore = getDrawerStore();
    const toastStore = getToastStore();

    //~ Toast Notifications
    import { Toast } from "@skeletonlabs/skeleton";

    //~ Popups
    import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
    import { storePopup } from "@skeletonlabs/skeleton";
    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    //~ Modals
    import { Modal } from "@skeletonlabs/skeleton";
    import type { ModalComponent } from "@skeletonlabs/skeleton";
    import EditorSettings from "$cmpt/modals/EditorSettings.svelte";
    import Login from "$cmpt/modals/Login.svelte";
    import CategoryInfo from "$cmpt/modals/CategoryInfo.svelte";

    export let data;

    const modalComponentRegistry: Record<string, ModalComponent> = {
        CategoryInfo: { ref: CategoryInfo },
        EditorSettings: { ref: EditorSettings },
        Login: { ref: Login },
    };

    //~ Error Handling
    const handleError = () => {
        toastStore.trigger({
            message: "An unexpected error occurred! Please reload the page to restore functionality.",
            background: "variant-filled-error",
        });
    };

    import { onMount } from "svelte";
    import { Cross1 } from "radix-icons-svelte";

    //~ Update Scroll Position
    let scrollElement: Element;
    const updateScroll = () => {
        document.body.style.setProperty("--scroll", scrollElement.scrollTop.toString());
    };

    onMount(() => {
        scrollElement = document.querySelector("#page");
    });

    const duration = 400;
</script>

<svelte:window on:error={handleError} />

<Toast />
<Modal components={modalComponentRegistry} />

{#key data.url}
    <slot />
{/key}

<Drawer regionDrawer="p-8">
    <div class="relative">
        <button
            class="absolute top-[-1.5em] right-[-1.5em]
            hover-highlight interactive p-1 rounded-full"
            on:click={() => drawerStore.close()}
        >
            <Cross1 size={24} />
        </button>
    </div>
    <h1 class="h1">Amplitude</h1>
    <p>epic temporary thingy lets goo</p>
</Drawer>
