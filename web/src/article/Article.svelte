<script lang="ts">
    import { onMount } from "svelte";
    import Quiz from "./Quiz.svelte";
    import Explorer from "./Explorer.svelte";
    import Outline from "./Outline.svelte";
    import { renderComponents, renderComponent } from "./article";
    import { fly } from "svelte/transition";
    // import { smoothAnchor } from "./article";
    import { urlQuery } from "../main";

    async function fetchArticle() {
        let e = new URLSearchParams();
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

    let name = "";
    let doc;
    let mount = false;

    fetchArticle().then((a) => {
        name = a.config.name;
        doc = new DOMParser().parseFromString(a.body, "text/html");
    });
    onMount(() => (mount = true));

    // after mounting & fetching the article, render it!
    let init = false;
    $: if (mount && doc) {
        renderComponent(doc.body, "Quiz", Quiz);
        renderComponents(doc.body);
        
        // replace all h2 with links to themselves
        doc.body.childNodes.forEach((element) => {
            if (element.localName != "h2") return;

            let id = element.textContent.toLowerCase().replace(/[^a-z0-9]/g, "-");
            element.id = id;
            element.innerHTML = `<a href="#${id}">${element.innerHTML}</a>`;
        });

        children = doc.body.childNodes;

        init = true;
    }

    let sidebars = false;
    let body_element: Element;
    let children: NodeListOf<ChildNode>;

    function transfer() {
        // console.log(children);
        body_element.replaceChildren(...children);
    }

    let flyOptions = { y: -20, duration: 300 };
</script>

{#if init}
    <div class="article">
        <div
            in:fly={flyOptions}
            on:introstart|once={transfer}
            on:introend|once={() => (sidebars = true)}
        >
            <div class="container body" bind:this={body_element}>
            </div>
        </div>
    </div>
{/if}
<div style:height="50vh" />

<style lang="scss">
    @use "variables";
    @use "../styles/mixins";

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
