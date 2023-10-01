<script lang="ts">
    import InputGroup from "$cmpt/form/InputGroup.svelte";
    import { createEventDispatcher } from "svelte";
    import { fetchApi } from "$lib/fetch";
    import { GitObject, type GitTree } from "./git";
    import { getToaster } from "$lib/toast";

    const dispatch = createEventDispatcher();
    const toaster = getToaster();

    let values = [null, "amplitude_articles", "main"];
    $: filled = values.every((s) => s?.length > 0);

    let disabled = false;
    const onClick = async () => {
        disabled = true;
        let [owner, repo, branch] = values;
        try {
            var res: GitTree = await fetchApi(`https://api.github.com/repos/${owner}/${repo}/git/trees/${branch}?recursive=1`, {
                headers: { "X-GitHub-Api-Version": "2022-11-28" },
            });
        } catch (e) {
            console.error(e);
            return;
        }
        let tree = new GitObject(res);
        
        // confirm the tree matches the expected structure

        dispatch("get_tree", tree);
    };
</script>

<p class="p">
    To get started, head on over to
    <a class="anchor" href="https://github.com/rcsc/amplitude_articles" target="_blank" rel="noreferrer noopener">
        https://github.com/rcsc/amplitude_articles
    </a>
    and fork it (leave the name as
    <code class="code">amplitude_articles</code>
    ). Then, ensure that you approve the
    <a class="anchor" href="https://github.com/apps/amplitude-editing-bot/installations/new" target="_blank" rel="noreferrer noopener">
        Github App
    </a>
    to access your forked repository by navigating to
    <code class="code whitespace-break-spaces">ACCOUNT > Only Select Repositories > ACCOUNT/amplitude_articles</code>
    . Finally, fill out the form below with the path to your repo to get started!
</p>
<InputGroup title="Github Branch to edit:" bind:values placeholders={["Owner", "Repository", "Branch"]} />
<div class="flex justify-end">
    <button class="btn variant-filled-primary" disabled={!filled || disabled} on:click={onClick}>Edit</button>
</div>
