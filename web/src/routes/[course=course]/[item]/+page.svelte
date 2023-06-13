<script lang="ts">
    import Article from "$cmpt/Article.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
    import Exercise from "./Exercise.svelte";

    export let data;
</script>

<AppShell slotPageContent="w-full h-full">
    <slot>
        {#if data.type == "article"}
            <Article {...data} classes="max-w-3xl" />
        {:else if data.type == "exercise"}
            <Exercise {data} />
        {/if}
    </slot>
</AppShell>
