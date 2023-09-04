<script lang="ts">
    export let data;

    //~ Themes
    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "../app.postcss";

    //~ `highlight.js`
    import hljs from "highlight.js";
    import { storeHighlightJs } from "@skeletonlabs/skeleton";
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
    import EditorSettings from "$cmpt/modals/EditorSettings.svelte";
    import Login from "$cmpt/modals/Login.svelte";
    const modalComponentRegistry: Record<string, ModalComponent> = {
        EditorSettings: {
            ref: EditorSettings,
        },
        Login: {
            ref: Login,
        },
    };

    //~ Error Handling
    const handleError = () => {
        toastStore.trigger({
            message:
                "An unexpected error occurred! Please reload the page to restore functionality.",
            background: "variant-filled-error",
        });
    };

    //~ Update Scroll Position
    let scrollElement: Element;
    const updateScroll = () => {
        document.body.style.setProperty(
            "--scroll",
            scrollElement.scrollTop.toString()
        );
    };

    import { AppShell, Drawer } from "@skeletonlabs/skeleton";
    import { HamburgerMenu, Cross1 } from "radix-icons-svelte";
    import NavBar from "$cmpt/NavBar.svelte";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    
    let loaded = false;
    onMount(() => {
        loaded = true;
        scrollElement = document.querySelector("#page");
    });
    
    let duration = 400;
</script>

<svelte:window on:error={handleError} />

<Toast />
<Modal components={modalComponentRegistry} />

<AppShell slotPageContent="relative" on:scroll={updateScroll}>
    <svelte:fragment slot="header">
        <NavBar path={data.pathname} />
    </svelte:fragment>

    {#if loaded}
        {#key data.pathname}
            <div
                class="h-full"
                in:fly={{ x: -5, duration, delay: duration }}
                out:fly={{ x: 5, duration }}
            >
                <slot />
            </div>
        {/key}
    {/if}
</AppShell>

<Drawer regionDrawer="p-8">
    <div class="relative">
        <button
            class="absolute top-[-1.5em] right-[-1.5em]
            hover:bg-slate-200 interactive p-1 rounded-full"
            on:click={() => drawerStore.close()}
        >
            <Cross1 size={24} />
        </button>
    </div>
    <h1 class="h1">Amplitude</h1>
    <p>epic temporary thingy lets goo</p>
</Drawer>
