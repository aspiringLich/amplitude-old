<script lang="ts">
  import Modal from "$cmpt/modals/Modal.svelte";

  const supported = fetch("/auth/supported").then((r) => r.json());
  //   const session = fetch("/auth/session").then((r) => r.json());
</script>

<Modal title="Login">
  {#await supported}
    <p>Loading auth options</p>
  {:then supported}
    <div class="flex flex-col gap-3">
      {#each supported as provider}
        <button
          on:click={() => (document.location = provider.path)}
          class="bg-blue-800"
        >
          Login with {provider.name}
        </button>
      {/each}
    </div>
  {/await}
</Modal>
