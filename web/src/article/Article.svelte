<script lang="ts">
    export let course: string;
    export let article: string;

    import { type ComponentType, onMount } from "svelte";
    import Quiz from "./Quiz.svelte";
    import { renderComponents, renderComponent } from "./article";

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

    let article_element: Element;
    let container_element: HTMLElement;
    onMount(() => {
        fetchDocument().then((doc) => {
            renderComponent(doc.body, "Quiz", Quiz, { course, article });
            renderComponents(doc.body, { course, article });
            article_element.replaceChildren(...doc.body.childNodes);

            container_element.style.visibility = "visible";
        });
    });
</script>

<div id="container" bind:this={container_element} style:visibility="hidden">
    <div bind:this={article_element} id="article" />
</div>
<div style:height="50vh" />

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
</style>
