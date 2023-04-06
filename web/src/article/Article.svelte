<script lang="ts">
    export let course: string;
    export let article: string;

    import { type ComponentType, onMount } from "svelte";
    import Quiz from "./Quiz.svelte";
    import Box from "../widgets/Box.svelte";

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

    let get = false;
    onMount(() => {
        fetchDocument().then((doc) => {
            renderComponent(doc, "Quiz", Quiz);
            get = true;
            article_element.replaceChildren(...doc.body.childNodes);
        });
    });
</script>

<div id="container" style={get ? "" : "visibility:hidden"}>
    <div bind:this={article_element} id="article" />
</div>

<style lang="scss">
    #container {
        width: clamp(300px, 80vw, 900px);
        margin: 0 auto;
        box-shadow: 0 0 16px rgba(0, 0, 0, 0.2);
        border: 1px solid hsl(0, 0, 90%);
        border-radius: 4px;
        padding: 16px;
        margin-top: 16px;
    }
</style>
