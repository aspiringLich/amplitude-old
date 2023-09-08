<script lang="ts">
    import NavBar from "$cmpt/NavBar.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import type { URL } from "url";

    export let path: [string, string][] = [];
    export let url: URL;

    //~ Update Scroll Position
    let scrollElement: Element;
    const updateScroll = () => {
        document.body.style.setProperty(
            "--scroll",
            scrollElement.scrollTop.toString()
        );
    };

    onMount(() => {
        scrollElement = document.querySelector("#page");
    });

    const duration = 400;
</script>

<AppShell slotPageContent="relative" on:scroll={updateScroll}>
    <svelte:fragment slot="header">
        <NavBar pathname={url.pathname} {path} />
    </svelte:fragment>

    <div
        class="h-full"
        in:fly={{ x: -5, duration, delay: duration }}
        out:fly={{ x: 5, duration }}
    >
        <slot />
    </div>
</AppShell>
