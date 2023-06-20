<script lang="ts">
    import { focusTrap, modalStore } from "@skeletonlabs/skeleton";
    import { Cross1 } from "radix-icons-svelte";

    export let title: string;
</script>

<div class="card bg-white w-modal shadow-xl relative" use:focusTrap={true}>
    <button
        class="btn btn-icon absolute top-1 right-1"
        on:click={() => modalStore.close()}
    >
        <Cross1 size={24} class="text-surface-900" />
    </button>

    <div class="max-h-[80vh] overflow-auto p-4 ">
        <h3 class="h3 !my-1">{title}</h3>
        <div class="settings grid grid-cols-1 gap-2 !mt-0">
            <slot />
        </div>
    </div>
</div>

<style lang="postcss">
    .settings {
        & :global(label) {
            @apply min-w-0;
        }

        & :global(.checkbox) {
            @apply mr-2;
        }
        
        /* & :global(.h2), :global(.h3), :global(.h4), :global(.h5) {
            @apply my-2;
        } */
    }
</style>
