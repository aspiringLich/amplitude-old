<script lang="ts">
    import type { TestResults } from "$lib/fetch";
    import type { ExerciseData } from "$lib/item";
    import Code from "$cmpt/article/Code.svelte";
    import { CrossCircled } from "radix-icons-svelte";

    export let data: ExerciseData;
    export let results: TestResults | Error | undefined = undefined;
    export let fn: string;

    $: func = data.config.functions[fn];
    $: res = results?.[fn];

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
                    class:incorrect={result?.type === "incorrect" ||
                        result?.type === "error"}
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
                                <CrossCircled class="stroke-error-800" />
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
                            <span class="my-auto">Inputs</span>
                            <Code code={stringify(test.inputs)} />
                            <span class="my-auto">Output</span>
                            <Code code={JSON.stringify(test.output, null, 2)} />

                            {#if result}
                                {#if result.output !== undefined}
                                    <span class="my-auto"> Recieved </span>
                                    <Code code={JSON.stringify(result.output, null, 2)} />
                                {/if}
                                <span class="my-auto">
                                    <code>stdout</code>
                                </span>
                                {#if result.stdout?.length === 0}
                                    <span class="font-normal"> N/A </span>
                                {:else}
                                    <Code code={result.stdout} />
                                {/if}
                                {#if result.traceback !== undefined}
                                    <span class="my-auto"> Traceback </span>
                                    <Code code={result.traceback} />
                                {/if}
                            {/if}
                            {#if !result}
                                <span class="col-span-2 my-auto">
                                    <span class="text-success-600-300-token">
                                        Run
                                    </span>
                                    your code to see more information!
                                </span>
                            {/if}
                        </div>
                    </th>
                </tr>
            </tfoot>
        {/if}
    </table>
</div>

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
            @apply text-success-700-200-token;
        }

        &.incorrect {
            @apply text-error-700-200-token;
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
