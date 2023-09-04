<script lang="ts">
    import { ArticleData, renderArticle } from "../../lib/item";
    import { onMount } from "svelte";

    export let data: ArticleData;
    export let classes = "";
    
    let body_element: HTMLElement;
    let padding = 1000;
    let init = false;

    onMount(() => {
        renderArticle(body_element, data);

        let hash = window.location.hash;
        if (hash) {
            let element: HTMLElement = body_element.querySelector("#" + hash.slice(1));
            if (element) {
                body_element.scrollTo(0, element.offsetTop + 20);
            }
        }

        init = true;
        padding = 50;
    });
</script>

<noscript>
    <style lang="postcss">
        .article {
            visibility: visible !important;
            opacity: 1 !important;
            transform: translateY(0) !important;
        }
    </style>
</noscript>

<div>
    <div class="article container-xl m-auto {classes}" class:show={init} bind:this={body_element}>
    <h1 class="text-5xl my-6">{@html data.title}</h1>
        {@html data.body}
    </div>
</div>

<!-- <div style:height="{padding}vh" /> -->

<!-- <div style:height="50vh" /> -->

<style lang="postcss">
    /*! purgecss start ignore */
    .article {
        visibility: hidden;
        opacity: 0;
        transform: translateY(-20px);
        transition: transform 0.5s ease-in-out, opacity 0.5s ease-in-out;

        &.show {
            transform: translateY(0);
            visibility: visible;
            opacity: 1;
        }
        
        &> :global(h2) {
            font-size: 1.75em;
            margin: 0.75em 0 0.75em 0;

            &:hover {
                text-decoration: underline 2px;
            }
        }

        & :global(h2 > a) {
            color: rgb(var(--text-color-base));
            text-decoration: none;
        }
    }
    
    :global(html.dark h2 > a) {
        color: rgb(var(--text-color-dark))
    }
</style>
