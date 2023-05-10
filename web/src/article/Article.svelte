<script lang="ts">
    import { onMount } from "svelte";
    import Quiz from "./Quiz.svelte";
    import Explorer from "./Explorer.svelte";
    import Outline from "./Outline.svelte";
    import { renderComponents, renderComponent } from "./article";
    import { fly } from "svelte/transition";
    import { smoothAnchor } from "./article";
    import { urlQuery } from "../main";

    async function fetchArticle() {
        let e = new URLSearchParams()
        const a = await fetch("/api/article", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ article: urlQuery().article }),
        });
        if (!a.ok) {
            throw new Error("failed to fetch article");
        }

        return a.json();
    }

    let title = "";
    let doc;
    let mount = false;

    fetchArticle().then((a) => {
        title = a.config.title;
        doc = new DOMParser().parseFromString(a.body, "text/html");
    });
    onMount(() => (mount = true));

    // after mounting & fetching the article, render it!
    let init = false;
    $: if (mount && doc) {
        renderComponent(doc.body, "Quiz", Quiz);
        renderComponents(doc.body);

        children = doc.body.childNodes;

        init = true;
    }

    // smoooth
    $: if (outline_element) {
        outline_element.querySelectorAll("a").forEach(smoothAnchor);
    }

    let sidebars = false;
    let body_element: Element;
    let outline_element: Element;
    let children: NodeListOf<ChildNode>;

    function transfer() {
        // console.log(children);
        body_element.replaceChildren(...children);

        document.querySelectorAll('a[href^="#"]').forEach(smoothAnchor);
    }

    function onResize() {
        width = window.innerWidth;
    }

    let flyOptions = { y: -20, duration: 300 };

    let width = window.innerWidth;
    $: outline = width >= 1100 && body_element != undefined;
</script>

<svelte:window on:resize={onResize} />

{#if init}
    <div class="article" data-margin={outline}>
        <div
            in:fly={flyOptions}
            on:introstart|once={transfer}
            on:introend|once={() => (sidebars = true)}
        >
            <div class="container heading">
                <h1>{title}</h1>
            </div>
            <div class="container body" bind:this={body_element} />
        </div>
        {#if outline && sidebars}
            <Outline article_body={body_element} />
        {/if}
    </div>
{/if}
<div style:height="50vh" />

<style lang="scss">
    @use "variables";
    @use "../styles/mixins";

    .article {
        display: flex;
        flex-direction: row;
        padding: 0 2em;

        &[data-margin="true"] {
            margin-right: variables.$outline-width !important;
        }
    }

    .container {
        @include mixins.box;

        margin: auto;
        margin-top: 16px;
        max-width: 740px;

        &.heading h1 {
            font-size: 3.5em;
            line-height: 100%;
        }
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
