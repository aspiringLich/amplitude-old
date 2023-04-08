<script lang="ts">
    import { fade } from "svelte/transition";

    export let article_body: Element;

    import { smoothAnchor } from "./article";

    class Heading {
        constructor(
            public id: string,
            public text: string,
            public element: HTMLElement
        ) {}
    }

    let headings: Heading[] = [];
    let positions: number[] = [];

    $: if (outline_element) {
        outline_element.querySelectorAll("a").forEach(smoothAnchor);
    }

    // the section were currently reading
    let reading = -1;
    const threshold = 0.2;

    // get the headings from the article
    let children = article_body.childNodes;
    for (let c of children) {
        if (!(c instanceof Element)) continue;

        let child = c as HTMLElement;
        let name = child.localName;

        if (name == "h2") {
            let id = child.textContent.toLowerCase().replace(/[^a-z0-9]/g, "_");

            child.id = id;
            child.innerHTML = `<a href="#${id}">${child.innerHTML}</a>`;
            headings.push(new Heading(id, child.textContent, child));
        }

        article_body.querySelectorAll("a").forEach(smoothAnchor);
    }
    positions = new Array(headings.length);

    // get the positions of the headings and find the one we're currently on
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

    let outline_element: Element;
</script>

<svelte:window
    on:scroll={calcHeadingPositions}
    on:resize={calcHeadingPositions}
/>

<div
    id="outline"
    bind:this={outline_element}
    in:fade={{ duration: 400, delay: 200 }}
    on:introend={calcHeadingPositions}
>
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

<style lang="scss">
    @use "variables.scss" as *;

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
</style>
