<script lang="ts">
    import Modal from "$cmpt/modals/Modal.svelte";
    import { getCategoryExercises, type CompletionConfig } from "$lib/fetch";
    import { getModalStore } from "@skeletonlabs/skeleton";

    const modalStore = getModalStore();
    const config: CompletionConfig = $modalStore[0].meta;
    const id: string = $modalStore[0].meta.id;
    
    let exercise = getCategoryExercises(id);
</script>

<Modal title={config.title}>
    {#await exercise then exercise}
        hee hee haw haw
    {/await}
</Modal>
