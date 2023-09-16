<script lang="ts">
    import Page from "$cmpt/Page.svelte";
    import InputGroup from "$cmpt/form/InputGroup.svelte";

    export let data;

    let values = [null, "amplitude_articles", "master"];
    let placeholders = ["Organization / User", "Repository", "Branch"];
    let [org, repo, branch] = values;

    $: filled = values.every((s) => s?.length > 0);
</script>

<Page url={data.url} class="floating-container">
    <div class="floating-container-inner">
        <h1 class="header h1">Editor</h1>
        <div class="body">
            <InputGroup title="Github Branch to edit:" bind:values {placeholders}>
                The github branch the web editor will use to edit the site.
            </InputGroup>
            <div class="flex justify-end">
                <button class="btn variant-filled-primary varian" disabled={!filled}>Edit</button>
            </div>
        </div>
    </div>
</Page>
