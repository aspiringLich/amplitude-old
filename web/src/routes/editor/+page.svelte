<script lang="ts">
    import Page from "$cmpt/Page.svelte";
    import { GitObject, type GitTree } from "./git";
    import Initial from "./Initial.svelte";
    
    export let data;
    
    const get_tree = (_tree: CustomEvent<GitTree>) => {
        let tree = _tree.detail;
        console.log(new GitObject(tree))
    };
</script>

<Page url={data.url} class="floating-container">
    <div class="floating-container-inner">
        <h1 class="header h1">Editor</h1>
        <div class="body">
            <Initial on:get_tree={get_tree}/>
        </div>
    </div>
</Page>
