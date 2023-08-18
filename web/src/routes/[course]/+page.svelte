<script lang="ts">
    import Article from "$cmpt/article/Article.svelte";
    import Code from "$cmpt/article/Code.svelte";

    export let data;
</script>

<div class="max-w-6xl mx-auto px-16">
    <div class="shadow-xl rounded-md">
        <div class="relative top-0 left-0 max-h-[24rem] overflow-hidden">
            <img class="w-full" src={data.course.icon} alt="Header" />
            <div class="absolute top-0 left-0" id="gradient" />
            <div class="absolute bottom-0 h-24 leading-none ml-4">
                <h1 class="h1 text-surface-50 !m-0 relative top-2">
                    {data.course.title}
                </h1>
                <b class="text-surface-50">{data.course.description}</b>
            </div>
        </div>
        <div class="bg-surface-50-900-token">
            <Article
                data={{
                    body: data.course.index,
                    title: "",
                }}
            />
        </div>
    </div>
</div>

<style lang="postcss">
    #gradient {
        width: 100%;
        height: 100%;
        background: linear-gradient(
            180deg,
            rgba(0, 0, 0, 0) 0%,
            rgba(0, 0, 0, 0) 50%,
            rgba(0, 0, 0, 0.5) 100%
        );
    }
</style>
