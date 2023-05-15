<script lang="ts">
    import { onMount } from "svelte";
    import { renderComponent } from "./article.js";

    import Code from "./Code.svelte";
    import Admonition from "./Admonition.svelte";

    export let data;

    let body: HTMLElement;
    let article: HTMLElement;
    let padding = 1000;

    onMount(() => {
        renderComponent(body, "pre", Code);
        renderComponent(body, "admonition", Admonition);

        // turn all h2s into links to themselves
        body.childNodes.forEach((element: HTMLElement) => {
            if (element.localName != "h2") return;

            let id = element.textContent
                .toLowerCase()
                .replace(/[^a-z0-9]/g, "-");
            element.id = id;
            element.innerHTML = `<a href="#${id}">${element.innerHTML}</a>`;
        });
        // scroll to the correct position
        let hash = window.location.hash;
        if (hash) {
            let element = document.getElementById(hash.slice(1));
            if (element) {
                window.scrollTo(0, element.offsetTop + 20)
            }
        }

        article.classList.add("show");
        padding = 50;
    });
</script>

<noscript>
    <style lang="scss">
        .article {
            visibility: visible !important;
            opacity: 1 !important;
            transform: translateY(0) !important;
        }

        noscript * {
            color: red;
        }
    </style>
    why you disable javascript :(
</noscript>
<div class="article" bind:this={article}>
    <div class="container body" bind:this={body}>
        <h1>{@html data.config.title}</h1>
        {@html data.body}
    </div>
</div>
<div style:height="{padding}vh" />

<!-- <div style:height="50vh" /> -->

<style lang="scss">
    .article {
        padding: 0 16px;

        visibility: hidden;
        opacity: 0;
        transform: translateY(-20px);
        transition: transform 0.5s ease-in-out, opacity 0.5s ease-in-out;

        &:global(.show) {
            transform: translateY(0);
            visibility: visible;
            opacity: 1;
        }
    }

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
