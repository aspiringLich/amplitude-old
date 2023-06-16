<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Editor from "$cmpt/Editor.svelte";
    import { itemID, type ExerciseData } from "$lib/item";
    import ExercisePanel from "./ExercisePanel.svelte";
    import type { TestResults } from "$lib/fetch";

    export let data: ExerciseData;

    let lang = "python";
    let code = data.lang_info[lang].code;
    let results: TestResults | Error | undefined;
    let run_disabled = false;

    async function run_code() {
        run_disabled = true;

        // wait to be ABSOLUTELY SURE the code is up to date
        await new Promise((r) => setTimeout(r, 100));

        let res = await fetch("/api/test", {
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
        } else {
            results = await res.json();
        }
        console.log(results)
        run_disabled = false;
    }
</script>

<Splitpanes theme="theme" class="p-16 max-w-6xl m-auto">
    <Pane minSize={38} size={58} class="relative flex">
        <ExercisePanel {data} bind:lang />
    </Pane>
    <Pane minSize={20} class="flex flex-col relative overflow-auto height-full">
        <div class="h-[43px] border-b border-surface-400 flex items-center">
            <button
                type="button"
                class="btn py-1 ml-1 variant-filled-primary left"
                disabled={run_disabled}
                on:click={run_code}
            >
                Run
            </button>
        </div>
        <div
            class="block relative height-full overflow-auto flex-[1_1_0px] box-content"
        >
            <Editor
                bind:value={code}
                bind:lang_name={lang}
                class="height-full overflow-auto"
            />
        </div>
    </Pane>
</Splitpanes>
