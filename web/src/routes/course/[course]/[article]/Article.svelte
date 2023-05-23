<script lang="ts">
    import { ArticleResponse, renderArticle } from "./article";
    import { onMount } from "svelte";

    export let data: ArticleResponse;

    let body: HTMLElement;
    let article: HTMLElement;
    let padding = 1000;

    onMount(() => {
        renderArticle(body);

        let hash = window.location.hash;
        if (hash) {
            let element: HTMLElement = body.querySelector("#" + hash.slice(1));
            if (element) {
                body.scrollTo(0, element.offsetTop + 20);
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
