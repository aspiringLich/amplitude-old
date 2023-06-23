<script lang="ts">
    import Article from "$cmpt/article/Article.svelte";
    import Code from "$cmpt/article/Code.svelte";
    import { stringify } from "postcss";
    import Exercise from "./Exercise.svelte";

    export let data;
</script>

{#if data.type == "article"}
    <Article {data} class="max-w-3xl" />
{:else if data.type == "exercise"}
    <Exercise {data} />
{:else}
    <h1>Item of type <code>{data.type}</code> NOT FOUND!</h1>
    <Code code={JSON.stringify(data, null, 2)} />
{/if}
