<script>
    export let course;
    export let article;

    import { onMount } from "svelte";
    import Quiz from "./Quiz.svelte";

    let article_element;

    // create a DOMParser from the html str
    async function fetchDOMParser() {
        const a = await fetch(`/api/course/${course}/${article}`);
        // if (!a.ok) {
        //     throw new Error("Failed to fetch article");
        // }
        let parser = new DOMParser();
        return parser.parseFromString(await a.text(), "text/html");
    }

    function renderComponent(dom, query, type) {
        let components = dom.querySelectorAll(query);
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
        fetchDOMParser().then((dom) => {
            renderComponent(dom, "Quiz", Quiz);
            article_element.replaceChildren(...dom.body.childNodes);
        });
    });
</script>

<div bind:this={article_element} id="article" />
