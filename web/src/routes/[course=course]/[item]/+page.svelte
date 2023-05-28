<script lang="ts">
    import Article from "./Article.svelte";

    export let data;
</script>

{#if data.type == "article"}
    <Article {data} />
{/if}
