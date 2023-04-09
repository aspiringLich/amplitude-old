<script lang="ts">
    export let course: string;
    export let track: string;
    export let article: string = undefined;

    import { onMount } from "svelte";
    import Quiz from "./Quiz.svelte";
    import { renderComponents, renderComponent } from "./article";
    import { fly } from "svelte/transition";
    import Outline from "./Outline.svelte";
    import { smoothAnchor } from "./article";

    // create a Document from the html str
    async function fetchDocument() {
        let map = {
            course,
            track,
        };
        if (article) {
            map["article"] = article;
        }
        const a = await fetch("/api/article", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(map),
        });
        if (!a.ok) {
            throw new Error("failed to fetch article");
        }

        let parser = new DOMParser();
        return parser.parseFromString(await a.text(), "text/html");
    }

    let heading = "";
    let init = false;
    let sidebars = false;

    let body_element: Element;
    let outline_element: Element;

    let children: NodeListOf<ChildNode>;

    onMount(() => {
        fetchDocument().then((doc) => {
            let title = doc.body.firstElementChild;
            console.assert(
                title.tagName == "H1",
                "loaded article does not have <h1> as its first element",
                title
            );
            heading = title.textContent;
            doc.body.removeChild(title);

            renderComponent(doc.body, "Quiz", Quiz, { course, article });
            renderComponents(doc.body, { course, article });

            children = doc.body.childNodes;

            init = true;
        });
    });

    $: if (outline_element) {
        outline_element.querySelectorAll("a").forEach(smoothAnchor);
    }

    // transfers a single element from the document to `article_element`
    function transfer() {
        body_element.replaceChildren(...children);

        document.querySelectorAll('a[href^="#"]').forEach(smoothAnchor);
    }

    function onResize() {
        width = window.innerWidth;
    }

    let flyOptions = { y: -20, duration: 300 };

    let width = window.innerWidth;
    $: right = width >= 1100 && body_element != undefined;
    $: left = width >= 700;
</script>

<svelte:window on:resize={onResize} />

{#if init}
    <div id="article" data-right={right} data-left={left}>
        <div id="left" />
        <div
            id="container"
            in:fly={flyOptions}
            on:introstart|once={transfer}
            on:introend|once={() => (sidebars = true)}
        >
            <h1>{heading}</h1>
            <div id="body" bind:this={body_element} />
        </div>
        {#if right && sidebars}
            <Outline article_body={body_element} />
        {/if}
    </div>
{/if}
<div style:height="50vh" />

<style lang="scss">
    @use "variables.scss" as *;
    // $article-width: calc(100vw - #{$outline-width} - #{$article-list-width});

    #left {
        position: fixed;
        float: left;
        top: 5px;
        left: 10px;
        height: 100%;
        width: $article-list-width;
    }

    #article {
        display: flex;
        flex-direction: row;
        padding: 0 2em;

        &[data-right="true"] {
            margin-right: $outline-width !important;
        }

        &[data-left="true"] {
            margin-left: $article-list-width !important;
        }
    }

    #container {
        margin: 0 auto;
        box-shadow: 0 0 16px rgba(0, 0, 0, 0.2);
        border: 1px solid hsl(0, 0%, 90%);
        border-radius: 10px;
        padding: 16px;
        margin-top: 16px;

        h1 {
            font-size: 3.5em;
            line-height: 100%;

            &:after {
                content: " ";
                display: block;
                border: 1px dashed black;
            }
        }
    }

    #body {
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
