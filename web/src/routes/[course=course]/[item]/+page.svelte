<script lang="ts">
    import Article from "$cmpt/Article.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
    import Exercise from "./Exercise.svelte";

    export let data;
</script>

<AppShell slotPageContent="w-full">
    <slot class="h-max">
        {#if data.type == "article"}
            <Article {...data} />
        {:else if data.type == "exercise"}
            <Exercise {data} />
        {/if}
    </slot>
</AppShell>
