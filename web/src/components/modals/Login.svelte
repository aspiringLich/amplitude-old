<script lang="ts">
    import Modal from "$cmpt/modals/Modal.svelte";
    import { supportedPlatforms } from "$lib/fetch";
    import { GithubLogo } from "radix-icons-svelte";

    const redirect = (platform) => (document.location = `${platform.path}?r=${encodeURIComponent(document.location.pathname)}`);
</script>

<Modal title="Login">
    <div class="flex flex-col gap-3 m-auto w-80">
        {#each supportedPlatforms as provider}
            <button on:click={() => redirect(provider)} class="btn variant-outline-surface bg-black rounded">
                {#if provider.name === "GitHub"}
                    <GithubLogo class="w-5 h-5 mr-2" />
                {:else if provider.name === "Google"}
                    <img src="/assets/img/google_logo.svg" alt="Google Logo" class="w-5 h-5 mr-2" />
                {/if}
                <span>Continue with {provider.name}</span>
            </button>
        {/each}
    </div>
</Modal>
