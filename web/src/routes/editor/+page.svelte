<script lang="ts">
    import Page from "$cmpt/Page.svelte";
    
    export let data;
</script>

<Page url={data.url} class="floating-container">
    <div class="floating-container-inner">
        <h1 class="header h1">Editor</h1>
        <div class="body">
            Gamin
        </div>
    </div>
</Page>
