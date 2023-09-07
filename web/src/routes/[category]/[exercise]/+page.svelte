<script lang="ts">
    import Exercise from "./Exercise.svelte";
    import Page from "$cmpt/Page.svelte";

    export let data;
</script>

<Page path={[["/categories", "categories"]]} pathname={data.pathname}>
    <Exercise {data} />
</Page>
