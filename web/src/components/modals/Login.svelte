<script lang="ts">
    import Modal from "$cmpt/modals/Modal.svelte";
    import { supportedPlatforms } from "$lib/fetch";
</script>

<Modal title="Login">
    <div class="flex flex-col gap-3">
        {#each supportedPlatforms as provider}
            <button on:click={() => (document.location = provider.path)} class="bg-blue-800">
                Login with {provider.name}
            </button>
        {/each}
    </div>
</Modal>
