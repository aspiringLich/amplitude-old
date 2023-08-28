<script lang="ts">
    import Article from "$cmpt/article/Article.svelte";
    import Code from "$cmpt/article/Code.svelte";
    import { stringify } from "postcss";
    import Exercise from "./Exercise.svelte";

    export let data;
</script>

<Exercise {data} />