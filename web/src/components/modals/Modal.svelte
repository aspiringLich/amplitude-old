<script lang="ts">
    import { focusTrap, getModalStore } from "@skeletonlabs/skeleton";
    import { Cross1 } from "radix-icons-svelte";
    
    const modalStore = getModalStore();
</script>

<div class="card w-modal shadow-xl relative" use:focusTrap={true}>
    <button
        class="absolute top-1 right-1 hover:bg-slate-200 interactive p-1 rounded-full"
        on:click={() => modalStore.close()}
    >
        <Cross1 size={20} class="stroke-3"/>
    </button>

    <div class="max-h-[80vh] overflow-auto p-4 pt-2">
        <slot />
    </div>
</div>
