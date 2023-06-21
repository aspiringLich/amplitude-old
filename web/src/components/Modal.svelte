<script lang="ts">
    import { focusTrap, modalStore } from "@skeletonlabs/skeleton";
    import { Cross1 } from "radix-icons-svelte";
</script>

<div class="card w-modal shadow-xl relative" use:focusTrap={true}>
    <button
        class="absolute top-2 right-2"
        on:click={() => modalStore.close()}
    >
        <Cross1 size={24} />
    </button>

    <div class="max-h-[80vh] overflow-auto p-4 ">
        <slot />
    </div>
</div>
