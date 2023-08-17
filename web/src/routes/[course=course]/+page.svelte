<script lang="ts">
    import Code from "$cmpt/article/Code.svelte";

    export let data;
</script>

<Code code={JSON.stringify(data, null, 2)} language="json" />
