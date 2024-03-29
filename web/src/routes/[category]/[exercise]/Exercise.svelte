<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Editor from "$cmpt/Editor.svelte";
    import { itemID } from "$lib/item";
    import ExercisePanel from "./ExercisePanel.svelte";
    import type { ExerciseData, TestResults } from "$lib/fetch";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import { Gear } from "radix-icons-svelte";
    import { editorSettings as settings } from "$lib/settings";
    import { onMount } from "svelte";
    import { getToaster } from "$lib/toast";

    const modalStore = getModalStore();
    const toaster = getToaster();

    export let data: ExerciseData;

    let lang = "python";
    let code = data.lang_info[lang].code;
    let results: TestResults | Error | undefined;
    let run_disabled = false;
    
    let tab_n: number;

    async function run_code() {
        run_disabled = true;

        let res = await fetch("/api/evaluate", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                lang,
                code,
                id: itemID(),
            }),
        });
        if (!res.ok) {
            results = new Error(await res.text());
            toaster.error("Error while trying to run code!");
            tab_n = 1;
        } else {
            results = (await res.json()) as TestResults;

            let passed = !Object.values(results).reduce((acc, x) => acc || !x.passed, false);
            if (passed) toaster.success("Congrats! All tests passed!");
            else toaster.error("Some tests failed!");
            tab_n = 1;
        }
        run_disabled = false;
    }

    $: fdir = $settings.flipPanes ? "!flex-row-reverse" : "!flex-row";
    $: show = loaded ? "show" : "";

    let loaded = false;
    onMount(() => {
        loaded = true;
    });
</script>

<Splitpanes theme="theme" class="floating-container {show} {fdir}" rtl={$settings.flipPanes}>
    <Pane minSize={20} class="relative flex shadow-xl">
        <ExercisePanel {data} bind:results bind:tab_n />
    </Pane>
    <Pane minSize={20} class="flex flex-col relative overflow-auto shadow-xl">
        <div class="h-[42px] bg-surface-200-700-token flex items-center justify-between {fdir}">
            <button type="button" class="btn py-1 ml-1 variant-filled-primary left" disabled={run_disabled} on:click={run_code}>Run</button>
            <button
                type="button"
                class="btn btn-icon hover:rotate-[22.5deg] text-surface-500 dark:text-surface-300"
                on:click={() => {
                    modalStore.trigger({
                        type: "component",
                        component: "EditorSettings",
                    });
                }}
            >
                <Gear size={24} />
            </button>
        </div>
        <div class="overflow-auto flex-[1_1_0px] bg-surface-50-900-token">
            <Editor bind:value={code} bind:lang_name={lang} class="overflow-auto fade-in" />
        </div>
    </Pane>
</Splitpanes>
