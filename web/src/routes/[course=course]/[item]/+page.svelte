<script lang="ts">
    import Article from "$cmpt/article/Article.svelte";
    import {
        AppShell,
        Drawer,
        drawerStore,
        type DrawerSettings,
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
    bg-surface-200/40 semi-interactive hover:bg-surface-200 hover:translate-x-3 active:bg-surface-400"
        on:click={() => drawerStore.open(settings)}
    >
        <ChevronRight color="black" size={24} class="ml-4" />
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
    <h1 class="h1">Amplitude</h1>
    <p>epic temporary thingy lets goo</p>
</Drawer>
