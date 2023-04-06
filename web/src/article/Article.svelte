<script lang="ts">
    export let course: string;
    export let article: string;

    import { type ComponentType, onMount } from "svelte";
    import Quiz from "./Quiz.svelte";

    let article_element: Element;

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
        // if (!a.ok) {
        //     throw new Error("Failed to fetch article");
        // }
        let parser = new DOMParser();
        return parser.parseFromString(await a.text(), "text/html");
    }

    function renderComponent(
        doc: Document,
        query: string,
        type: ComponentType
    ) {
        let components = doc.querySelectorAll(query);
        for (const item of components) {
            let props = {
                course,
                article,
            };
            for (const attr of item.attributes) {
                props[attr.name] = attr.value;
            }
            new type({
                target: item,
                props,
            });
        }
    }

    onMount(() => {
        fetchDocument().then((doc) => {
            renderComponent(doc, "Quiz", Quiz);
            article_element.replaceChildren(...doc.body.childNodes);
        });
    });
</script>

<div bind:this={article_element} id="article" />
