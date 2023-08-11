<script lang="ts">
    export let data;

    //~ Themes
    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "@skeletonlabs/skeleton/styles/skeleton.css";
    import "../app.postcss";
    import "../themes/base.postcss";
    import "../themes/colors.postcss";

    //~ `highlight.js`
    import hljs from "highlight.js";
    import { storeHighlightJs, toastStore } from "@skeletonlabs/skeleton";
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
    import EditorSettings from "$cmpt/modals/EditorSettings.svelte";
    import Login from "$cmpt/modals/Login.svelte";
    const modalComponentRegistry: Record<string, ModalComponent> = {
        EditorSettings: {
            ref: EditorSettings,
        },
        Login: {
            ref: Login
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

    import { AppShell, Drawer, drawerStore } from "@skeletonlabs/skeleton";
    import { ChevronRight, Cross1 } from "radix-icons-svelte";
    import NavBar from "$cmpt/NavBar.svelte";
    import { fly } from "svelte/transition";
</script>

<svelte:window on:error={handleError} />

<Toast />
<Modal components={modalComponentRegistry} />

<AppShell slotPageContent="relative">
    <svelte:fragment slot="header">
        <NavBar path={data.pathname} />
    </svelte:fragment>
    <!-- OH LAWD -->
    <button
        class="w-14 h-10 p-2 rounded-r-full fixed top-16 left-[-24px]
        bg-surface-200/40 hover:bg-surface-200 active:bg-surface-400
        dark:bg-surface-800/40 dark:hover:bg-surface-800 dark:active:bg-surface-600
        semi-interactive hover:translate-x-3"
        on:click={() =>
            drawerStore.open({
                width: "w-96",
            })}
    >
        <ChevronRight size={24} class="ml-4" />
    </button>

    {#key data.pathname}
        <div
            class="h-full"
            in:fly={{ x: -10, duration: 500, delay: 500 }}
            out:fly={{ x: 5, duration: 500 }}
        >
            <slot />
        </div>
    {/key}
</AppShell>

<Drawer regionDrawer="p-8">
    <div class="relative">
        <button
            class="absolute top-[-1.5em] right-[-1.5em] semi-interactive 
            hover:bg-slate-200 interactive p-1 rounded-full"
            on:click={() => drawerStore.close()}
        >
            <Cross1 size={24} />
        </button>
    </div>
    <h1 class="h1">Amplitude</h1>
    <p>epic temporary thingy lets goo</p>
</Drawer>
