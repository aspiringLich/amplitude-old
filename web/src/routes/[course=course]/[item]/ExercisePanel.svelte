<script lang="ts">
    import Article from "$cmpt/Article.svelte";
    import type { TestResults } from "$lib/fetch";
    import type { ExerciseData } from "$lib/item";
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";

    export let data: ExerciseData;
    export const results: TestResults | Error | undefined = undefined;
    export let lang: string = "";

    $: fn_list = Object.keys(data.config.functions);

    let tab_lp = 1;
    let run_disabled = false;

    let selected_fn: string | undefined;
</script>

<TabGroup
    class="flex grow flex-col relative overflow-auto height-full"
    regionPanel="height-full overflow-auto flex-[1_0_0px] !mt-0"
>
    <Tab bind:group={tab_lp} name="instructions" value={0}>Instructions</Tab>
    <Tab bind:group={tab_lp} name="test" value={1}>Test Cases</Tab>

    <span slot="panel">
        {#if tab_lp == 0}
            <Article
                classes="h-max"
                title={data.config.title}
                body={data.config.instructions}
            />
        {:else if tab_lp == 1}
            {#each fn_list as fn}
                <div class="table-container">
                    <table class="table table-hover">
                        <thead>
                            <tr>
                                <th>Inputs</th>
                                <th>Output</th>
                                <th>Recieved</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each data.config.functions[fn].tests as func, i}
                                {@const output = func.output}
                                <tr>
                                    <td>
                                        {func.inputs
                                            .map((x) => x.toString())
                                            .join(", ")}
                                    </td>
                                    <td>{output}</td>
                                    <td />
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/each}
        {/if}
    </span>
</TabGroup>

<style lang="postcss">
    tbody td {
        text-overflow: ellipsis;
    }
    
    table {
        background-color: transparent !important;
    }
    
    tr {
        border-width: 0 !important;
    }
    
    
</style>
