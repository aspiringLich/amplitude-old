<script lang="ts">
    import Admonition from "$cmpt/article/Admonition.svelte";
    import Article from "$cmpt/article/Article.svelte";
    import Code from "$cmpt/article/Code.svelte";
    import type { TestResults } from "$lib/fetch";
    import type { ExerciseData } from "$lib/item";
    import { TabGroup, Tab, popup } from "@skeletonlabs/skeleton";
    import type { PopupSettings } from "@skeletonlabs/skeleton";
    import { CrossCircled } from "radix-icons-svelte";

    export let data: ExerciseData;
    export let results: TestResults | Error | undefined = undefined;

    let tabN = 0;

    $: fn_list = Object.keys(data.config.functions);

    function popupSettings(fn: string, i: number): PopupSettings {
        return {
            event: "click",
            target: `popup-${fn}-${i}`,
            placement: "left",
        };
    }

    function stringify(list: Object[]): string {
        return list.map((x) => JSON.stringify(x, null, 2)).join(", ");
    }

    let selected = undefined;
    function select(n: number) {
        if (selected === n) {
            selected = undefined;
        } else {
            selected = n;
        }
    }
</script>

<TabGroup
    class="flex grow flex-col overflow-auto"
    border="border-b-0"
    regionList="bg-surface-200 dark:bg-surface-800"
    regionPanel="overflow-auto flex-[1_0_0px] !mt-0 px-4 bg-surface-50 dark:bg-surface-900"
>
    <Tab bind:group={tabN} name="instructions" value={0}>Instructions</Tab>
    <Tab bind:group={tabN} name="test" value={1}>Test Cases</Tab>

    <svelte:fragment slot="panel">
        {#if tabN == 0}
            <Article
                class="h-max"
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
                        class="interactable hover:cursor-pointer"
                        class:correct={result?.type === "correct"}
                        class:incorrect={result?.type ===
                            "incorrect" || result?.type === "error"}
                        on:click={() => select(i)}
                        class:selected={selected === i}
                    >
                        <td>
                            {stringify(test.inputs)}
                        </td>
                        <td>{JSON.stringify(test.output)}</td>
                        <td>
                            {#if result}
                                {#if result.type === "correct"}
                                    {JSON.stringify(test.output)}
                                {:else if result.type === "incorrect"}
                                    {JSON.stringify(result.output)}
                                {:else if result.type === "error"}
                                    <CrossCircled
                                        class="stroke-error-800"
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
            {#if selected !== undefined}
                {@const test = func.tests[selected]}
                {@const result = res?.results[selected]}
                <tfoot>
                    <tr>
                        <th colspan="3" class="normal-case">
                            <div
                                class="grid grid-cols-[6em_1fr] grid-flow-row gap-2"
                            >
                                <span class="my-auto">
                                    inputs
                                </span>
                                <Code
                                    code={stringify(test.inputs)}
                                />
                                <span class="my-auto">
                                    expected
                                </span>
                                <Code
                                    code={JSON.stringify(
                                        test.output,
                                        null,
                                        2
                                    )}
                                />

                                {#if result}
                                    <span class="my-auto">
                                        stdout
                                    </span>
                                    {#if result.stdout?.length === 0}
                                        <span class="font-normal">
                                            N/A
                                        </span>
                                    {:else}
                                        <Code
                                            code={result.stdout}
                                        />
                                    {/if}
                                {/if}
                                {#if !result}
                                    <span
                                        class="col-span-2 my-auto"
                                    >
                                        <span
                                            class="text-success-800"
                                        >
                                            Run
                                        </span> Your code to see more
                                        information!
                                    </span>
                                {/if}
                            </div>
                        </th>
                    </tr>
                </tfoot>
            {/if}
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

    thead {
        @apply border-b-0;
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

        &.selected {
            background-color: rgb(var(--color-surface-500) / 0.3);
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
