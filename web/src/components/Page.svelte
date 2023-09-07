<script lang="ts">
    import NavBar from "$cmpt/NavBar.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
    import { getContext, onMount } from "svelte";
    import { fly } from "svelte/transition";
    
    export let path: [string, string][] = [];
    export let pathname: string;
    
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
        {#key pathname}
            <NavBar pathname={pathname} path={path} />
        {/key}
    </svelte:fragment>

    {#key pathname}
        <div
            class="h-full"
            in:fly={{ x: -5, duration, delay: duration }}
            out:fly={{ x: 5, duration }}
        >
            <slot />
        </div>
    {/key}
</AppShell>
