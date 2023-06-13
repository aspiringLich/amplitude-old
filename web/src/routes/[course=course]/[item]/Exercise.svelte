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
    <Pane minSize={38} size={58}>
        <ExercisePanel {data} bind:code bind:lang />
    </Pane>
    <Pane minSize={20} class="flex flex-col">
        <!-- <div class="h-[45.64px] border-b border-surface-400">
    <button type="button" class="btn variant-filled-primary">Run</button>
</div> -->
        <Editor bind:value={code} lang_name={lang} />
    </Pane>
</Splitpanes>
