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

    let width = window.innerWidth;
    $: right = width > 1000;
    $: left = width > 700;
</script>

<svelte:window on:resize={() => (width = window.innerWidth)} />

<div id="article" data-right={right} data-left={left}>
    {#if init}
        <div id="left" />
        <div
            id="container"
            in:fly={{ y: -100, easing: quadInOut, duration: 800 }}
            on:introstart={transfer}
        >
            <h1>{heading}</h1>
            <div id="body" bind:this={body_element} />
        </div>
        {#if right}
            <div id="right">
                <h2>Outline</h2>
            </div>
        {/if}
    {/if}
</div>
<div style:height="50vh" />

<style lang="scss">
    $right_sidebar_width: 200px;
    $left_sidebar_width: clamp(200px, 20%, 300px);

    #left {
        position: fixed;
        float: left;
        top: 5px;
        left: 10px;
        height: 100%;
        width: $left_sidebar_width;
    }

    #right {
        position: fixed;
        top: 5px;
        right: 10px;
        float: right;
        height: 100%;
        width: $right_sidebar_width;
    }

    #article {
        display: flex;
        flex-direction: row;

        &[data-right="true"] {
            margin-right: $right_sidebar_width;
        }

        &[data-left="true"] {
            margin-left: $left_sidebar_width;
        }
    }

    #container {
        width: clamp(300px, 90%, 900px);
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
