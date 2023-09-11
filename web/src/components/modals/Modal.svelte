<script lang="ts">
    import { focusTrap, getModalStore } from "@skeletonlabs/skeleton";
    import { Cross1 } from "radix-icons-svelte";
    
    const modalStore = getModalStore();
    
    export let title: string
    export let width: string = "w-modal";
</script>

<div class="{width} shadow-xl relative pt-0" use:focusTrap={true}>
    <button
        class="absolute top-1 right-1 hover-highlight interactive p-1 rounded-full"
        on:click={() => modalStore.close()}
    >
        <Cross1 size={20} class="stroke-3"/>
    </button>
    <div class="card-header py-0 bg-surface-200-700-token rounded-t-container-token pt-1">
        <h2 class="h2 !my-0">{title}</h2>
    </div>
    <div class="max-h-[80vh] overflow-auto p-4 pt-2 bg-surface-50-900-token rounded-b-container-token">
        <slot />
    </div>
</div>
