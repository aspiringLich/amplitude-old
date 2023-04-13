<script>
  import Admonition from "../article/Admonition.svelte";
  import Icon from "../widgets/Icon.svelte";

  const session = fetch("/auth/session").then((r) => r.json());
</script>

<div class="container">
  <nav class="shadow">
    <div class="logo">⚡ amplitude</div>
    <div class="sep" />
    <div class="page active">Home</div>
    <div class="page">Corses</div>
    <div class="page">About</div>
    <div class="user">
      {#await session then session}
        {#if !session.error}
          <img src={session.avatar} alt="User avatar" />
        {:else}
          <Icon icon="login" size="1.5em" />
        {/if}
      {/await}
    </div>
  </nav>

  <div class="content shadow">
    <h1>Home</h1>
    <p>Welcome to amplitude, The learning platform for the 23rd century.</p>

    <Admonition type="info">
      <p>
        This application is currently still <em>in development</em>. Feel free
        to check out the
        <a href="github.com/aspiringLich/amplitude">source code</a> on Github.
      </p>
    </Admonition>
  </div>
</div>

<style lang="scss">
  :global(body) {
    margin: 0;
    padding: 0;
  }

  .shadow {
    box-shadow: 0 10px 30px -10px rgba(0, 0, 0, 0.1);
  }

  nav {
    color: #334155;
    display: flex;
    justify-content: start;
    align-items: center;
    margin-bottom: 1.5rem;

    border: 3px solid #eeeeee;
    padding: 10px 20px 10px 20px;

    max-width: 1280px;
    width: 50%;
    left: 50%;
    transform: translateX(-50%);
    position: relative;

    & .logo {
      font-size: 1.5rem;
      cursor: pointer;
      margin-right: 0.375rem * 2;
    }

    & .sep {
      border-left: 1px solid #eeeeee;
      border-right: 1px solid #eeeeee;
      border-radius: 0.25rem;
      height: 1.5rem;
    }

    & .page {
      margin-left: 0.375rem;
      padding: 0.375rem;

      &:hover {
        border-radius: 0.25rem;
        cursor: pointer;
        background: #eeeeee;
      }
    }

    & .user {
      margin-left: auto;
      display: flex;

      &:hover {
        border-radius: 0.25rem;
        cursor: pointer;
        background: #eeeeee;
      }

      & :global(span) {
        padding: calc((36px - 24px) / 2);
      }

      & img {
        border-radius: 0.25rem;
        width: 36px;
        height: 36px;
      }
    }
  }

  .container {
    background: #fbfbfb;
    padding-top: 1.5rem;
    min-height: calc(100vh - 1.5rem);

    & .content {
      border: 3px solid #eeeeee;
      padding: 0.75 rem;
      padding: 10px 20px 10px 20px;

      max-width: 1280px;
      width: 50%;
      left: 50%;
      transform: translateX(-50%);
      position: relative;
    }
  }
</style>
