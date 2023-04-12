<script>
  const supported = fetch("/auth/supported").then((r) => r.json());
  const session = fetch("/auth/session").then((r) => r.json());

  const logout = () => (document.location = "/auth/logout");
  const login = (provider) =>
    (document.location = `/auth/${provider}/redirect`);
</script>

<p>im using this until we build the home screen</p>

{#await supported}
  <p>Loading auth options</p>
{:then supported}
  {#if supported.github}
    <a href="/auth/github/redirect" on:click={() => login("github")}
      >Login with Github</a
    >
  {/if}
  {#if supported.google}
    <br />
    <a href="/auth/google/redirect" on:click={() => login("google")}
      >Login with Google</a
    >
  {/if}
{/await}

<hr />

{#await session}
  <p>Loading session</p>
{:then session}
  {#if !session.error}
    <p>Logged in as {session.name}</p>
    <img src={session.avatar} alt="User avatar" width="100px" />
    <br />
    <a href="/auth/logout" on:click={logout} on:keypress={logout}>Logout</a>
  {:else}
    <p>Not logged in</p>
  {/if}
{/await}
