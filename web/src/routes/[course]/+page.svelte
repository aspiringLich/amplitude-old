<script lang="ts">
    import Code from "$cmpt/article/Code.svelte";

    export let data;
</script>

<div class="max-w-6xl mx-auto px-16">
    <div class="relative top-0 left-0 h-[24rem] overflow-hidden">
        <img class="w-full" src={data.course.icon} alt="Header" />
    </div>
</div>
