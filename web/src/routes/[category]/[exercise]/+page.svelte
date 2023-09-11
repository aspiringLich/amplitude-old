<script lang="ts">
    import Exercise from "./Exercise.svelte";
    import Page from "$cmpt/Page.svelte";

    export let data;
</script>

<Page path={[["/categories", "categories"]]} url={data.url}>
    <Exercise {data} />
</Page>
