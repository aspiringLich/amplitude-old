<script lang="ts">
    import Article from "$cmpt/article/Article.svelte";
    import {
        AppShell,
        Drawer,
        drawerStore,
        type DrawerSettings,
        LightSwitch,
    } from "@skeletonlabs/skeleton";
    import Exercise from "./Exercise.svelte";
    import { ChevronRight, Cross1 } from "radix-icons-svelte";

    export let data;

    const settings: DrawerSettings = {
        width: "w-96",
    };
</script>

<AppShell>
    <button
        class="w-14 h-10 p-2 rounded-r-full absolute top-16 left-[-24px]
    bg-surface-200/40 hover:bg-surface-200  active:bg-surface-400
    dark:bg-surface-800/40 dark:hover:bg-surface-800 dark:active:bg-surface-600
    semi-interactive hover:translate-x-3
    dark:text-white"
        on:click={() => drawerStore.open(settings)}
    >
        <ChevronRight size={24} class="ml-4" />
    </button>
    {#if data.type == "article"}
        <Article {data} class="max-w-3xl" />
    {:else if data.type == "exercise"}
        <Exercise {data} />
    {/if}
</AppShell>

<Drawer regionDrawer="p-8">
    <div class="relative">
        <button
            class="absolute top-[-1em] right-[-1em] semi-interactive"
            on:click={() => drawerStore.close()}
        >
            <Cross1 size={24} />
        </button>
    </div>
    <LightSwitch />
    <h1 class="h1">Amplitude</h1>
    <p>epic temporary thingy lets goo</p>
</Drawer>
