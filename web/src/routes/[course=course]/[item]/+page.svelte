<script lang="ts">
    import Article from "$cmpt/article/Article.svelte";
    import Exercise from "./Exercise.svelte";

    export let data;
</script>

{#if data.type == "article"}
    <Article {data} class="max-w-3xl" />
{:else if data.type == "exercise"}
    <Exercise {data} />
{/if}
