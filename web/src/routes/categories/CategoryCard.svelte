<script lang="ts">
    import type { CompletionConfig } from "$lib/fetch";
    import { ProgressRadial } from "@skeletonlabs/skeleton";
    import { getModalStore } from "@skeletonlabs/skeleton";

    export let config: CompletionConfig;
    export let id: string;

    const modalStore = getModalStore();

    $: percent = (config.completed.length / config.exercises.length) * 100;

    const open = () => {
        modalStore.trigger({
            type: "component",
            component: "CategoryInfo",
            meta: {...config, id: id },
        });
    };
</script>

<div class="card card-hover flex flex-grow hover:cursor-pointer" on:click={open} role="none">
    <section class="m-4 mr-0 pr-4 border-surface-600-300-token border-r-[1px]">
        <ProgressRadial width="w-[3.25em]" value={percent} stroke={100} font={150}>
            {percent}%
        </ProgressRadial>
    </section>
    <section class="py-4 pl-2 flex gap-x-4">
        <div class="flex flex-col">
            <h2 class="text-xl font-bold truncate">
                {config.title}
            </h2>
            <i class="font-light truncate">
                {config.description + "dkjaofaklfjak"}
            </i>
        </div>
    </section>
</div>
