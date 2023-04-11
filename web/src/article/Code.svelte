<script lang="ts">
    import { onMount } from "svelte";
    import hijs from "highlight.js/lib/common";
    import Icon from "../widgets/Icon.svelte";
    import Button from "../widgets/Button.svelte";

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

<div id="code" class="hljs">
    <code id="lines" bind:this={lines_element} />
    <div
        id="grid"
        on:mouseenter={() => (copy_button = true)}
        on:mouseleave={() => (copy_button = false)}
    >
        <div id="container">
            <pre bind:this={pre_element}><slot /></pre>
        </div>
        {#if copy_button}
            <div id="copy">
                <Button color="1" onclick={copy} padding={1}>
                    <Icon icon="content_copy" color="black" />
                </Button>
            </div>
        {/if}
    </div>
</div>

<style lang="scss">
    #code {
        display: flex;
        flex-direction: row;
        align-items: flex-start;
        border-radius: 0.3em;
    }
    
    #grid {
        position: relative;
        // overflow: scroll;
        display: grid;
        width: 100%;
    }

    #lines {
        padding: 1em 0.5em;
        display: block;
        border-right: 1px solid rgba(255, 255, 255, 0.5);

        user-select: none;
    }

    #copy {
        position: absolute;
        top: 5px;
        right: 5px;
        display: flex;
        justify-content: end;
    }

    :global(.n-top-border-radius) #code {
        border-top-left-radius: 0;
        border-top-right-radius: 0;
    }

    :global(.n-border-radius) #code {
        border-radius: 0;
    }

    #container {
        display: flex;
        flex-direction: row;
        overflow: auto;

        pre {
            flex-shrink: 1;
            white-space: pre;
            line-height: 1.1em;
            display: table;
            overflow: auto;

            :global(code) {
                overflow: auto;
                border-radius: 0.3em;
            }
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
