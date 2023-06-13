<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Editor from "$cmpt/Editor.svelte";
    import Article from "$cmpt/Article.svelte";
    import { ExerciseData } from "$lib/item";
    
    export let data: ExerciseData;
</script>

<Splitpanes theme="theme">
    <Pane minSize={38} size={58}>
        <Article
            classes="h-max max-w-xl"
            title={data.config.title}
            body={data.config.instructions}
        />
    </Pane>
    <Pane minSize={20}>
        <Editor />
    </Pane>
</Splitpanes>
