<script lang="ts">
    import { Pane, Splitpanes } from 'svelte-splitpanes';
    import Article from "../../../components/Article.svelte";

    export let data;
</script>

{#if data.type == "article"}
    <Article {data} />
{/if}
