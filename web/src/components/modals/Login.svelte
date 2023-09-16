<script lang="ts">
    import Modal from "$cmpt/modals/Modal.svelte";
    import { supportedPlatforms } from "$lib/fetch";
</script>

<Modal title="Login">
    <div class="flex flex-col gap-3 m-auto w-80">
        {#each supportedPlatforms as provider}
            <button on:click={() => (document.location = provider.path)} class="btn variant-outline-surface">
                Login with {provider.name}
            </button>
        {/each}
    </div>
</Modal>
