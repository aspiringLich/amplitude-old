<script lang="ts">
    import Page from "$cmpt/Page.svelte";
    import { ProgressRadial } from "@skeletonlabs/skeleton";
    import CategoryCard from "./CategoryCard.svelte";

    export let data;
</script>

<Page url={data.url} class="floating-container">
    <div class="floating-container-inner">
        <h1 class="header h1">Categories</h1>
        <div class="body">
            <div class="flex gap-y-4 flex-col">
                {#each Object.entries(data.categories) as [category, config]}
                    <CategoryCard {config} id={category} />
                {/each}
            </div>
        </div>
    </div>
</Page>
