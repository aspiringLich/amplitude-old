<script lang="ts">
    import Article from "$cmpt/Article.svelte";
    import type { ExerciseData } from "$lib/item";
    import {
        TabGroup,
        Tab,
        Table,
        tableMapperValues,
    } from "@skeletonlabs/skeleton";
    import type { TableSource } from "@skeletonlabs/skeleton";
    import { TestResults, postCode } from "$lib/fetch";
    import Admonition from "$cmpt/Admonition.svelte";

    export let data: ExerciseData;
    export let code: string;
    export let lang: string;

    $: fn_list = Object.keys(data.config.functions);

    let tab_lp = 0;
    let run_disabled = false;
    let results: TestResults | undefined = undefined;

    let selected_fn: string | undefined;

    async function run_code() {
        run_disabled = true;

        // wait to be ABSOLUTELY SURE the code is up to date
        await new Promise((r) => setTimeout(r, 100));

        let res: TestResults = await postCode(code, lang);
        results = res;
        run_disabled = false;
    }
</script>

<TabGroup
    class="flex grow flex-col relative overflow-auto height-full"
    regionPanel="height-full overflow-auto flex-[1_0_0px] !mt-0"
>
    <Tab bind:group={tab_lp} name="instructions" value={0}>Instructions</Tab>
    <Tab bind:group={tab_lp} name="test" value={1}>Test Cases</Tab>

    <svelte:fragment slot="panel">
        {#if tab_lp == 0}
            <Article
                classes="h-max"
                title={data.config.title}
                body={data.config.instructions}
            />
        {:else if tab_lp == 1}
            <button
                class="btn variant-filled-primary"
                type="button"
                on:click={run_code}
                disabled={run_disabled}>Run</button
            >

            {#each fn_list as fn}
                {#each data.config.functions[fn].tests as func, i}
                    <div class="card w-40 h-80 border-none">
                        <header class="card-header text-lg font-bold">
                            Test Case {i}
                        </header>
                        <section class="p-4" />
                    </div>
                {/each}
            {/each}
        {/if}
    </svelte:fragment>
</TabGroup>
