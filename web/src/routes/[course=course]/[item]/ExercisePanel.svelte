<script lang="ts">
    import Admonition from "$cmpt/Admonition.svelte";
    import Article from "$cmpt/Article.svelte";
    import Code from "$cmpt/Code.svelte";
    import type { TestResults } from "$lib/fetch";
    import type { ExerciseData } from "$lib/item";
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";
    import { CrossCircled } from "radix-icons-svelte";

    export let data: ExerciseData;
    export let results: TestResults | Error | undefined = undefined;
    let tabN = 0;

    $: fn_list = Object.keys(data.config.functions);
    $: console.log(results);
</script>

<TabGroup
    class="flex grow flex-col relative overflow-auto height-full"
    regionPanel="height-full overflow-auto flex-[1_0_0px] !mt-0 px-4"
>
    <Tab bind:group={tabN} name="instructions" value={0}>Instructions</Tab>
    <Tab bind:group={tabN} name="test" value={1}>Test Cases</Tab>

    <svelte:fragment slot="panel">
        {#if tabN == 0}
            <Article
                classes="h-max"
                title={data.config.title}
                body={data.config.instructions}
            />
        {:else if tabN == 1}
            {#if results instanceof Error}
                <Admonition type="error" container="p-0">
                    <Code code={results.message} rounded="rounded-t-none"/>
                </Admonition>
            {/if}

            {#each fn_list as fn}
                {@const func = data.config.functions[fn]}
                {@const res = results?.[fn]}

                <div class="table-container mt-4">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>Inputs</th>
                                <th>Output</th>
                                <th>Recieved</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each func.tests as test, i}
                                {@const result = res?.results[i]}
                                <tr
                                    class="interactable"
                                    class:correct={result?.type === "correct"}
                                    class:incorrect={result?.type ===
                                        "incorrect" || result?.type === "error"}
                                >
                                    <td>
                                        {test.inputs
                                            .map((x) => x.toString())
                                            .join(", ")}
                                    </td>
                                    <td>{test.output}</td>
                                    <td>
                                        {#if result}
                                            {#if result.type === "correct"}
                                                {test.output}
                                            {:else if result.type === "incorrect"}
                                                {result.output.toString()}
                                            {:else if result.type === "error"}
                                                <CrossCircled
                                                    class="stroke-error-500"
                                                />
                                            {/if}
                                        {/if}
                                    </td>
                                </tr>
                            {/each}
                            <tr
                                class:correct={res?.hidden}
                                class:incorrect={res?.hidden === false}
                            >
                                <td colspan={3}>
                                    ...{func.hidden_cases} more hidden cases
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            {/each}
        {/if}
    </svelte:fragment>
</TabGroup>

<style lang="postcss">
    tbody td {
        word-wrap: break-word;
        text-overflow: ellipsis;
        @apply py-2;
        font-family: monospace;
    }

    table {
        background-color: transparent !important;
    }

    .table tbody tr {
        border-width: 0 !important;

        &.correct {
            color: rgb(var(--color-success-800));
        }

        &.incorrect {
            color: rgb(var(--color-error-800));
        }
    }

    .table tbody tr.interactable {
        &:hover {
            background-color: rgb(var(--color-surface-500) / 0.2);
        }

        &:active {
            background-color: rgb(var(--color-surface-500) / 0.25);
        }
    }

    .table tbody tr.interactable:nth-child(2n) {
        &:hover {
            background-color: rgb(var(--color-surface-500) / 0.25);
        }

        &:active {
            background-color: rgb(var(--color-surface-500) / 0.3);
        }
    }
</style>
