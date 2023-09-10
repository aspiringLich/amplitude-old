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

<Modal title={config.title}>
    <div class="mb-2">{config.description}</div>
    {#await exercise then exercise}
        {#each Object.entries(exercise) as [exercise_id, e]}
            {@const complete = config.completed.includes(exercise_id)}
            {@const incomplete = config.incomplete.includes(exercise_id)}
            <a class="p-2 card card-hover flex flex-row items-center rounded-full" href="/{exercise_id}" on:click={close}>
                <div class="w-[5.75em]">
                    {#if complete}
                        <span class="badge variant-ghost-primary">Completed</span>
                    {:else if incomplete}
                        <span class="badge variant-ghost-secondary">In Progress</span>
                    {:else}
                        <span class="badge variant-ghost-surface">Not Started</span>
                    {/if}
                </div>
                <span class="text-xl font-semibold !ml-2">{e.title}</span>
                
            </a>
        {/each}
    {/await}
</Modal>
