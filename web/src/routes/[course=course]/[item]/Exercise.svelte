<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Editor from "$cmpt/Editor.svelte";
    import type { ExerciseData } from "$lib/item";
    import ExercisePanel from "./ExercisePanel.svelte";

    export let data: ExerciseData;

    let lang = "python";
    let code = data.lang_info[lang].code;
</script>

<Splitpanes theme="theme" class="p-16 max-w-6xl m-auto">
    <Pane minSize={38} size={58} class="relative flex">
        <ExercisePanel {data} bind:code bind:lang />
    </Pane>
    <Pane minSize={20} class="flex flex-col relative overflow-auto height-full">
        <div class="h-[43px] border-b border-surface-400" />
        <div class="block relative height-full overflow-auto flex-[1_1_0px] box-content">
            <Editor bind:value={code} lang_name={lang} class="height-full overflow-auto" />
        </div>
    </Pane>
</Splitpanes>
