<script lang="ts">
    import { onMount } from "svelte";
    import hijs from "highlight.js/lib/common";
    import { fade, fly } from "svelte/transition";
    import Icon from "../widgets/Icon.svelte";

    // import assert from "assert";

    let pre_element: HTMLElement;
    let lines_element: HTMLElement;
    let code: HTMLElement;

    onMount(() => {
        code = pre_element.firstElementChild as HTMLElement;
        code.classList.add("nglobal");
        console.assert(code.tagName == "CODE", code);

        let lines = 0;
        for (let c of code.textContent) {
            if (c == "\n") lines++;
        }

        let line_string = "";
        for (let i = 0; i < lines; i++) {
            line_string += `${i + 1}<br />`;
        }

        lines_element.innerHTML = line_string;
        hijs.highlightElement(code as HTMLElement);
    });

    let copy_button = false;

    function copy() {
        navigator.clipboard.writeText(code.innerText);
    }
</script>

<div
    style:position="relative"
    style:overflow-y="hidden"
    on:mouseenter={() => (copy_button = true)}
    on:mouseleave={() => (copy_button = false)}
>
    <div id="container" class="hljs">
        <code id="lines" bind:this={lines_element} />
        <pre bind:this={pre_element}><slot /></pre>
    </div>
    {#if copy_button}
        <div id="copy">
            <Icon
                type="button"
                args={{ onclick: copy }}
                background="rgba(255, 255, 255, 0.5)"
            >
                content_copy
            </Icon>
        </div>
    {/if}
</div>

<style lang="scss">
    #copy {
        position: absolute;
        top: 5px;
        right: 5px;
    }

    #container {
        display: flex;
        flex-direction: row;
        border-radius: 0.3em;

        pre {
            overflow-x: auto;
            :global(code) {
                border-radius: 0.3em;
            }
        }

        #lines {
            padding: 1em 0.5em;
            display: block;
            border-right: 1px solid rgba(255, 255, 255, 0.5);

            user-select: none;
        }

        :global(code) {
            font-family: "Jet Brains Mono";
            line-height: 1.1em;
            font-size: 0.85em;
        }
    }

    pre {
        margin: 0;
    }
</style>
