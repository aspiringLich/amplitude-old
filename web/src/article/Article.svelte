<script lang="ts">
    export let course: string;
    export let article: string;

    import { type ComponentType, onMount } from "svelte";
    import Quiz from "./Quiz.svelte";
    import { renderComponents, renderComponent } from "./article";
    import { fly } from "svelte/transition";
    import { quadInOut } from "svelte/easing";

    // create a Document from the html str
    async function fetchDocument() {
        const a = await fetch("/api/article", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                course,
                article,
            }),
        });
        if (!a.ok) {
            throw new Error("failed to fetch article");
        }

        let parser = new DOMParser();
        return parser.parseFromString(await a.text(), "text/html");
    }

    let heading = "";
    let init = false;

    let body_element: Element;
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

    // transfers a single element from the document to `article_element`
    function transfer() {
        body_element.replaceChildren(...children);
    }
</script>

<div id="article">
    {#if init}
        <div
            id="container"
            in:fly={{ y: -100, easing: quadInOut, duration: 800 }}
            on:introstart={transfer}
        >
            <h1>{heading}</h1>
            <div
                id="body"
                bind:this={body_element}
            />
        </div>
    {/if}
    <div style:height="50vh" />
</div>

<style lang="scss">
    #container {
        width: clamp(300px, 80vw, 900px);
        margin: 0 auto;
        box-shadow: 0 0 16px rgba(0, 0, 0, 0.2);
        border: 1px solid hsl(0, 0%, 90%);
        border-radius: 10px;
        padding: 16px;
        margin-top: 16px;
    }

    h1 {
        font-size: 3em;
        line-height: 100%;

        &:after {
            content: " ";
            display: block;
            border: 1px dashed black;
        }
    }
</style>
