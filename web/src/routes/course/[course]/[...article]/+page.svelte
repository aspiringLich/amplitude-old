<script lang="ts">
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { renderComponent } from "./article.js";
    import Code from "./Code.svelte";
    import Admonition from "./Admonition.svelte";
    // import { smoothAnchor } from "./article";

    export let data;

    let html = `<h1>${data.name}</h1>${data.body}`;
    let body: HTMLElement;

    onMount(() => {
        body.innerHTML = html;
        renderComponent(body, "pre", Code);
        renderComponent(body, "Admonition", Admonition);
    });

    let flyOptions = { y: -20, duration: 300 };
</script>

<div class="article">
    <div in:fly={flyOptions}>
        <div class="container body" bind:this={body} />
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
