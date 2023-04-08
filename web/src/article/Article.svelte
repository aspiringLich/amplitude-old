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
    let outline_element: Element;

    let children: NodeListOf<ChildNode>;
    let headings: Heading[] = [];
    let positions: number[] = [];

    // the section were currently reading
    let reading = -1;
    const threshold = 0.1;
    
    $: console.log(reading)

    function calcHeadingPositions() {
        for (let i = 0; i < headings.length; i++) {
            let heading = headings[i];
            positions[i] = heading.element.getBoundingClientRect().top;
        }
        reading = positions.findIndex(
            (p) => p > window.innerHeight * threshold
        );
        if (reading == -1) reading = positions.length;
        reading -= 1;
    }

    class Heading {
        constructor(
            public id: string,
            public text: string,
            public element: HTMLElement
        ) {}
    }

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

            let ids = {};

            for (let c of children) {
                if (!(c instanceof Element)) continue;

                let child = c as HTMLElement;
                let name = child.localName;
                if (["h2"].includes(name)) {
                    let id = child.textContent
                        .toLowerCase()
                        .replace(/[^a-z0-9]/g, "_");
                    headings.push(new Heading(id, child.textContent, child));

                    child.id = id;
                    child.innerHTML = `<a href="#${id}">${child.innerHTML}</a>`;
                }
            }
            positions = new Array(headings.length);

            init = true;
        });
    });

    function smoothAnchor(anchor: HTMLElement) {
        anchor.addEventListener("click", function (e) {
            e.preventDefault();

            document.querySelector(this.getAttribute("href")).scrollIntoView({
                behavior: "smooth",
            });
        });
    }

    $: if (outline_element) {
        outline_element.querySelectorAll("a").forEach(smoothAnchor);
    }

    // transfers a single element from the document to `article_element`
    function transfer() {
        body_element.replaceChildren(...children);

        document.querySelectorAll('a[href^="#"]').forEach(smoothAnchor);
        calcHeadingPositions();
    }

    function onResize() {
        calcHeadingPositions();
        width = window.innerWidth;
    }

    function onScroll() {
        calcHeadingPositions();
    }

    let flyOptions = { y: -100, easing: quadInOut, duration: 400 };

    let width = window.innerWidth;
    $: right = width >= 1100;
    $: left = width >= 700;
</script>

<svelte:window on:resize={onResize} on:scroll={onScroll} />

{#if init}
    <div id="article" data-right={right} data-left={left}>
        <div id="left" />
        <div id="container" in:fly={flyOptions} on:introstart|once={transfer}>
            <h1>{heading}</h1>
            <div id="body" bind:this={body_element} />
        </div>
        {#if right}
            <div id="outline" bind:this={outline_element}>
                <h1>Outline</h1>
                <ul>
                    {#each headings as heading, i}
                        <li id="item" data-on={i == reading}>
                            <a href={"#" + heading.id}>
                                {heading.text}
                            </a>
                        </li>
                    {/each}
                </ul>
            </div>
        {/if}
    </div>
{/if}
<div style:height="50vh" />

<style lang="scss">
    $outline-width: 270px;
    $article-list-width: clamp(200px, 20%, 300px);
    // $article-width: calc(100vw - #{$outline-width} - #{$article-list-width});

    #outline {
        position: fixed;
        top: 16px;
        right: 0;
        float: right;
        height: 100%;
        width: $outline-width;
        padding-right: 10px;
        padding-left: 1em;

        ul {
            padding-left: 0;
            margin: 0;
        }

        h1 {
            font-size: 1.5em;
        }

        li {
            list-style: none;
            padding: 0.25em 0 0.25em 1em;
            border-color: black;
            border-left: 4px solid;
            margin-left: 4px;
            transition: 0.5s;
            
            &[data-on="true"] {
                border-color: var(--color-blue-d2);
                background-color: var(--color-blue-l1);
            }
        }

        a {
            text-decoration: none;
            color: var(--color-0-d2);

            &:hover {
                color: var(--color-blue-d2);
            }
        }
    }

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
