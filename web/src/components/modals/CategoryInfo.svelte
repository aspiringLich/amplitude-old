<script lang="ts">
    import Modal from "$cmpt/modals/Modal.svelte";
    import { getCategoryExercises, type CompletionConfig } from "$lib/fetch";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import { CheckCircled, Circle, MinusCircled } from "radix-icons-svelte";

    const modalStore = getModalStore();
    const config: CompletionConfig = $modalStore[0].meta;
    const id: string = $modalStore[0].meta.id;

    let exercise = getCategoryExercises(id);

    const close = () => {
        modalStore.close();
    };
</script>

<Modal title={config.title} width="w-modal-wide">
    <div class="mb-2">{config.description}</div>
    {#await exercise}
        {#each config.exercises as _}
            <div class="p-2 card flex flex-row items-center rounded-full h-11">
                <div class="w-[5.75em] placeholder animate-pulse" />
                <div class="ml-2 w-full placeholder animate-pulse" />
            </div>
        {/each}
    {:then exercise}
        <div class="grid grid-cols-1 md:grid-cols-2">
            {#each Object.entries(exercise) as [exercise_id, e]}
                {@const complete = config.completed.includes(exercise_id)}
                {@const incomplete = config.incomplete.includes(exercise_id)}
                <a class="p-2 card flex flex-row items-center rounded-full" href="/{exercise_id}" on:click={close}>
                    <span class="w-[5.75em]">
                        {#if complete}
                            <span class="badge variant-ghost-primary">Completed</span>
                        {:else if incomplete}
                            <span class="badge variant-ghost-secondary">In Progress</span>
                        {:else}
                            <span class="badge variant-ghost-surface">Not Started</span>
                        {/if}
                    </span>
                    <span class="text-xl font-semibold !ml-2">{e.title}</span>
                </a>
            {/each}
        </div>
    {/await}
</Modal>
