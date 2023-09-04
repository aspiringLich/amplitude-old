<script lang="ts">
    import Admonition from "$cmpt/article/Admonition.svelte";
    import Article from "$cmpt/article/Article.svelte";
    import Code from "$cmpt/article/Code.svelte";
    import type { ExerciseData } from "$lib/item";
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";
    import type { TestResults as TypeTestResult } from "$lib/fetch";
    import TestResults from "./TestResults.svelte";
    
    export let data: ExerciseData;
    export let results: TypeTestResult | Error | undefined = undefined;
    
    let tabN = 0;

    $: fn_list = Object.keys(data.config.functions);

    // function popupSettings(fn: string, i: number): PopupSettings {
    //     return {
    //         event: "click",
    //         target: `popup-${fn}-${i}`,
    //         placement: "left",
    //     };
    // }
</script>

<TabGroup
    class="flex grow flex-col overflow-auto"
    border="border-b-0"
    regionList="bg-surface-200-700-token"
    regionPanel="overflow-auto flex-[1_0_0px] !mt-0 px-4 bg-surface-50 dark:bg-surface-900"
>
    <Tab bind:group={tabN} name="instructions" value={0}>Instructions</Tab>
    <Tab bind:group={tabN} name="test" value={1}>Test Cases</Tab>

    <svelte:fragment slot="panel">
        {#if tabN == 0}
            <Article
                classes="h-max px-4 py-2"
                data={{
                    title: data.config.title,
                    body: data.config.instructions,
                }}
            />
        {:else if tabN == 1}
            {#if results instanceof Error}
                <Admonition type="error" container="p-0">
                    <Code code={results.message} rounded="rounded-t-none" />
                </Admonition>
            {/if}

            {#each fn_list as fn}
                <TestResults bind:data bind:results bind:fn />
            {/each}
        {/if}
    </svelte:fragment>
</TabGroup>
