<script lang="ts">
    import ExplorerList from "./ExplorerList.svelte";
    import { TrackConfig, urlPath } from "./article";

    // create a Document from the html str
    async function fetchList() {
        let path = urlPath();
        const a = await fetch("/api/article_list", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                course: path[1],
                track: path[2],
            }),
        });

        if (!a.ok) {
            throw new Error("failed to fetch article list!");
        }

        return a;
    }

    let track: Promise<TrackConfig> = fetchList().then((a) => a.json());
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
