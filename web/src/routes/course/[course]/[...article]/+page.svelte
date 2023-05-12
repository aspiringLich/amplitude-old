<script lang="ts">
    import { fly } from "svelte/transition";
    import Admonition from "./Admonition.svelte";
    import Code from "./Code.svelte";
    import { renderComponent } from "./article";
    // import { smoothAnchor } from "./article";

    export let data;

    let name = data.name;
    
    let body_element: HTMLElement;

    // after mounting & fetching the article, render it!
    $: if (body_element) {
        renderComponent(body_element, "pre", Code);
        renderComponent(body_element, "Admonition", Admonition);

        // replace all h2 with links to themselves
        body_element.childNodes.forEach((element: HTMLElement) => {
            if (element.localName != "h2") return;

            let id = element.textContent
                .toLowerCase()
                .replace(/[^a-z0-9]/g, "-");
            element.id = id;
            element.innerHTML = `<a href="#${id}">${element.innerHTML}</a>`;
        });
    }

    let flyOptions = { y: -20, duration: 300 };
</script>

<div class="article">
    <div
        in:fly={flyOptions}
    >
        <div class="container body" bind:this={body_element} />
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
