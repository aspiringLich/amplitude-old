<script lang="ts">
    import { onMount } from "svelte";
    import hijs from "highlight.js/lib/common";
    import { Copy } from "radix-icons-svelte";

    // import assert from "assert";

    let pre_element: HTMLElement;
    let code: HTMLElement;

    onMount(() => {
        code = pre_element.firstElementChild as HTMLElement;
        code.classList.add("nglobal");
        console.assert(code.tagName == "CODE", code);
        
        hijs.highlightElement(code as HTMLElement);
    });

    let copy_button = false;

    function copy() {
        navigator.clipboard.writeText(code.innerText);
    }
</script>

<div class="code hljs">
    <!-- <code class="lines" bind:this={lines_element} /> -->
    <div
        class="grid"
        on:mouseenter={() => (copy_button = true)}
        on:mouseleave={() => (copy_button = false)}
    >
        <div class="container">
            <pre bind:this={pre_element}><slot /></pre>
        </div>
        {#if copy_button}
            <button class="hljs copy" on:click={copy}>
                <Copy />
            </button>
        {/if}
    </div>
</div>

<style lang="scss">    
    .copy {
        position: absolute;
        bottom: 5px;
        right: 5px;
        display: flex;
        justify-content: end;
        padding: 0;
        border: none;
        
        cursor: pointer;
        
        &:active {
            color: var(--gray-medium);
        }
    }

    .code {
        display: flex;
        flex-direction: row;
        align-items: flex-start;
        border-radius: 0.3em;
        margin: 0.75em 0;
    }

    .grid {
        position: relative;
        // overflow: scroll;
        display: grid;
        width: 100%;
    }

    .container {
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
            
            padding: 0.25em 1em;
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
