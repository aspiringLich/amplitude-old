<script lang="ts">
    import Page from "$cmpt/Page.svelte";
    import CategoryCard from "./CategoryCard.svelte";

    export let data;
</script>

<Page url={data.url} class="floating-container">
    <div class="floating-container-inner">
        <h1 class="header h1">Categories</h1>
        <div class="body">
            <div class="grid grid-flow-row-dense">
                {#each Object.entries(data.categories) as [category, config]}
                    <CategoryCard id={category} config={config} />
                {/each}
            </div>
        </div>
    </div>
</Page>
