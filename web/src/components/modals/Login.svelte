<script lang="ts">
  import Modal from "$cmpt/modals/Modal.svelte";

  const supported = fetch("/auth/supported").then((r) => r.json());
  const session = fetch("/auth/session").then((r) => r.json());
</script>

<Modal title="Login">
  {#await session}
    <p>Loading session</p>
  {:then session}
    <p>
      Logged in as <strong>{session.name}</strong> through {session.platform}
    </p>
    <img src={session.avatar} alt="User avatar" width="100px" />
    <br />
    <button on:click={() => (document.location = "/auth/logout")}>
      Logout
    </button>
  {:catch}
    {#await supported}
      <p>Loading auth options</p>
    {:then supported}
      <div class="">
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
  {/await}
</Modal>
