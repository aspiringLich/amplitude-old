<script lang="ts">
    import { urlParts } from "../main";
    import ExplorerList from "./ExplorerList.svelte";

    // create a Document from the html str
    async function fetchList() {
        const a = await fetch("/api/article-list", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                course: urlParts()[0]
            }),
        });

        if (!a.ok) {
            throw new Error("failed to fetch article list!");
        }

        return a;
    }    
</script>

<div id=container>
    <!-- {#await track then track}
        <ExplorerList entries={track.children}></ExplorerList>
    {:catch error}
        <span style:color=red>Could not fetch article list :(</span>
        {error}
    {/await} -->
</div>

<style>
    #container {
        position: fixed;
        left: 0;
        top: 0;
    }
</style>
