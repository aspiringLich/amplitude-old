<script lang="ts">
    import { focusTrap, modalStore } from "@skeletonlabs/skeleton";
    import { Cross1 } from "radix-icons-svelte";
</script>

<div class="card bg-white w-modal shadow-xl relative" use:focusTrap={true}>
    <button
        class="btn btn-icon absolute top-1 right-1"
        on:click={() => modalStore.close()}
    >
        <Cross1 size={24} class="text-surface-900" />
    </button>

    <div class="max-h-[80vh] overflow-auto p-4 ">
        <slot />
    </div>
</div>
