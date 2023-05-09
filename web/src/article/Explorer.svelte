<script lang="ts">
    import ExplorerList from "./ExplorerList.svelte";

    // create a Document from the html str
    async function fetchList() {
        const a = await fetch("/api/article_list", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                
            }),
        });

        if (!a.ok) {
            throw new Error("failed to fetch article list!");
        }

        return a;
    }
    
    let track;
    // let track: Promise<TrackConfig> = fetchList().then((a) => a.json());
</script>

<div id=container>
    {#await track then track}
        <ExplorerList entries={track.children}></ExplorerList>
    {:catch error}
        <span style:color=red>Could not fetch article list :(</span>
        {error}
    {/await}
</div>

<style>
    #container {
        position: fixed;
        left: 0;
        top: 0;
    }
</style>
