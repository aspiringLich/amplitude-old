<script lang="ts">
    import Page from "$cmpt/Page.svelte";

    export let data;
</script>

<Page pathname={data.pathname}>
    <div class="floating-container">
        <div class="floating-container-inner">
            <h1 class="header h1">Categories</h1>
            <div class="body">
                <div class="grid grid-flow-row-dense">daflakj</div>
            </div>
        </div>
    </div>
</Page>
