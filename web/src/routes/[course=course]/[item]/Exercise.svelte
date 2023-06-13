<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Editor from "$cmpt/Editor.svelte";
    import type { ExerciseData } from "$lib/item";
    import ExercisePanel from "./ExercisePanel.svelte";

    export let data: ExerciseData;

    let lang = "python";
    let code = data.lang_info[lang].code;
</script>

<Splitpanes theme="theme">
    <Pane minSize={38} size={58}>
        <ExercisePanel {data} bind:code={code} bind:lang={lang} />
    </Pane>
    <Pane minSize={20}>
        <Editor bind:value={code} lang_name={lang} />
    </Pane>
</Splitpanes>
