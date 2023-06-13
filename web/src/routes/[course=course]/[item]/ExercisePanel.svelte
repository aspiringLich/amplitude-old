<script lang="ts">
    import Article from "$cmpt/Article.svelte";
    import type { ExerciseData } from "$lib/item";
    import { TabGroup, Tab, Table, tableMapperValues } from "@skeletonlabs/skeleton";
    import type { TableSource } from "@skeletonlabs/skeleton";
    import { TestResults, postCode } from "$lib/fetch";
    import Admonition from "$cmpt/Admonition.svelte";

    export let data: ExerciseData;
    export let code: string;
    export let lang: string;

    let tab_lp = 0;

    let run_disabled = false;
    let results: undefined | TestResults = undefined;
    // $: body = results?.results?.map((result, i) => {
    //     return {
    //         position: (i + 1).toString(),
    //         result: result.type,
    //     };
    // });
    // $: source = {
    //     head: ["#", "Result"],
    //     body: tableMapperValues(body, ["position", "result"]),
    //     meta: tableMapperValues(body, ["position", "result"]),
    // };

    async function run_code() {
        run_disabled = true;

        // wait to be ABSOLUTELY SURE the code is up to date
        await new Promise((r) => setTimeout(r, 100));

        let res = await postCode(code, lang);
        console.log(res);
        results = res;
        run_disabled = false;
    }
</script>

<TabGroup>
    <Tab bind:group={tab_lp} name="article" value={0}>Article</Tab>
    <Tab bind:group={tab_lp} name="article" value={1}>Run</Tab>

    <svelte:fragment slot="panel">
        {#if tab_lp == 0}
            <Article
                classes="h-max"
                title={data.config.title}
                body={data.config.instructions}
            />
        {:else if tab_lp == 1}
            <div class="px-2 pb-2">
                <button
                    class="btn variant-filled-primary"
                    type="button"
                    on:click={run_code}
                    disabled={run_disabled}>Run</button
                >

                {#if results}
                    <Admonition type={results.passed ? "correct" : "incorrect"}>
                        {results.passed ? "You passed woo" : "You failed :("}
                    </Admonition>
                {/if}
            </div>
        {/if}
    </svelte:fragment>
</TabGroup>
