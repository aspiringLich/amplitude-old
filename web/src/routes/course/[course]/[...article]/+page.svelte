<script lang="ts">
    import { onMount } from "svelte";
    import { renderComponent } from "./article.js";
    import { fly } from "svelte/transition";

    import Code from "./Code.svelte";
    import Admonition from "./Admonition.svelte";

    export let data;

    let body: HTMLElement; 
    onMount(() => {
        renderComponent(body, "pre", Code);
        renderComponent(body, "admonition", Admonition);
    });

    let flyOptions = { y: -20, duration: 300 };
</script>

<div class="article">
    <div in:fly={flyOptions}>
        <div class="container body" bind:this={body}>
            <h1>{@html data.config.title}</h1>
            {@html data.body}
        </div>
    </div>
</div>
<div style:height="50vh" />

<style lang="scss">
    @use "variables";

    .container {
        margin: auto;
        margin-top: 16px;
        max-width: 740px;
    }

    .body {
        > :global(h2) {
            font-size: 1.75em;
            margin: 0.75em 0 0.75em 0;

            &:hover {
                text-decoration: underline 2px;
            }
        }

        :global(h2 > a) {
            color: black;
            text-decoration: none;
        }
    }
</style>
