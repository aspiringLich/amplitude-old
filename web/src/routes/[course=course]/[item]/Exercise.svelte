<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Editor from "$cmpt/Editor.svelte";
    import { itemID, type ExerciseData } from "$lib/item";
    import ExercisePanel from "./ExercisePanel.svelte";
    import type { TestResults } from "$lib/fetch";
    import { toastStore } from "@skeletonlabs/skeleton";
    import type { ToastSettings } from "@skeletonlabs/skeleton";
    import Code from "$cmpt/Code.svelte";
    import { Gear } from "radix-icons-svelte";
    import colors from "tailwindcss/colors";

    export let data: ExerciseData;

    let lang = "python";
    let code = data.lang_info[lang].code;
    let results: TestResults | Error | undefined;
    let run_disabled = false;

    async function run_code() {
        run_disabled = true;

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
            const t: ToastSettings = {
                message: "Error while trying to run code!",
                background: "variant-filled-error",
            };
            toastStore.trigger(t);
        } else {
            results = (await res.json()) as TestResults;

            let passed = !Object.values(results).reduce(
                (acc, x) => acc || !x.passed,
                false
            );
            const t: ToastSettings = passed
                ? {
                      message: "Congrats! All tests passed!",
                      background: "variant-filled-success",
                  }
                : {
                      message: "Some tests failed!",
                      background: "variant-filled-error",
                  };
            toastStore.trigger(t);
        }
        run_disabled = false;
    }

    function stringify(list: Object[]): string {
        return list.map((x) => JSON.stringify(x, null, 2)).join(", ");
    }
</script>

<Splitpanes theme="theme" class="p-16 max-w-6xl m-auto">
    <Pane minSize={20} class="relative flex">
        <ExercisePanel {data} bind:results />
    </Pane>
    <Pane minSize={20} class="flex flex-col relative overflow-auto height-full">
        <div class="h-[42px] bg-surface-200 flex items-center justify-between">
            <button
                type="button"
                class="btn py-1 ml-1 variant-filled-primary left"
                disabled={run_disabled}
                on:click={run_code}
            >
                Run
            </button>
            <button type="button" class="btn btn-icon hover:rotate-[22.5deg]">
                <Gear size={24} color={colors["slate"][500]} />
            </button>
        </div>
        <div
            class="overflow-auto flex-[1_1_0px]"
        >
            <Editor
                bind:value={code}
                bind:lang_name={lang}
                class="overflow-auto"
            />
        </div>
    </Pane>
</Splitpanes>
