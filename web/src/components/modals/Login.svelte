<script lang="ts">
    import Modal from "$cmpt/modals/Modal.svelte";
    import { supportedPlatforms } from "$lib/fetch";

    const redirect = (platform) => (document.location = `${platform.path}?r=${encodeURIComponent(document.location.pathname)}`);
</script>

<Modal title="Login">
    <div class="flex flex-col gap-3 m-auto w-80">
        {#each supportedPlatforms as provider}
            <button on:click={() => redirect(provider)} class="btn variant-outline-surface">
                Login with {provider.name}
            </button>
        {/each}
    </div>
</Modal>
