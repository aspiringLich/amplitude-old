<script>
    import Redirecting from "../widgets/Redirecting.svelte";

    const supported = fetch("/auth/supported").then((r) => r.json());
    const session = fetch("/auth/session").then((r) => r.json());

    let redirecting = null;
    const redirect = (location) => {
        redirecting = location;
        document.location = location;
    };
</script>

{#if redirecting}
    <Redirecting location={redirecting} />
{:else}
    <p>im using this until we build the home screen</p>

    {#await session}
        <p>Loading session</p>
    {:then session}
        {#if !session.error}
            <p>
                Logged in as <strong>{session.name}</strong> through {session.platform}
            </p>
            <img src={session.avatar} alt="User avatar" width="100px" />
            <br />
            <button on:click={() => redirect("/auth/logout")}>Logout</button>
        {:else}
            {#await supported}
                <p>Loading auth options</p>
            {:then supported}
                {#each supported as provider}
                    <button on:click={() => redirect(provider.path)}>
                        Login with {provider.name}
                    </button>
                {/each}
            {/await}
        {/if}
    {/await}
{/if}

<style>
    button {
        display: block;
        margin-top: 0.5em;
    }
</style>
