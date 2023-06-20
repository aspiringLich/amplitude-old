<!-- Modified from https://github.com/skeletonlabs/skeleton/blob/master/packages/skeleton/src/lib/utilities/CodeBlock/CodeBlock.svelte -->

<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { Copy, Check } from "radix-icons-svelte";
    import { storeHighlightJs } from "@skeletonlabs/skeleton";
    import { clipboard } from "@skeletonlabs/skeleton";
    import { fade } from "svelte/transition";

    // Event Handler
    const dispatch = createEventDispatcher();

    // Types
    import type { CssClasses } from "@skeletonlabs/skeleton";

    // Props
    /** Sets a language alias for Highlight.js syntax highlighting. */
    export let language = "plaintext";
    /** Provide the code snippet to render. Be mindful to escape as needed! */
    export let code = "";

    /** Specify if line numbers should be added to the code block*/
    export let lineNumbers = false;
    /** Specify if a copy button should appear on the code block */
    export let copy = false;

    // Props (styles)
    /** Provide classes to set the background color. */
    export let background: CssClasses = "bg-neutral-900/90";
    /** Provided classes to set the backdrop blur. */
    export let blur: CssClasses = "";
    /** Provide classes to set the text size. */
    export let text: CssClasses = "text-sm";
    /** Provide classes to set the text color. */
    export let color: CssClasses = "text-white";
    /** Provide classes to set the border radius. */
    export let rounded: CssClasses = "rounded-container-token";
    /** Provide classes to set the box shadow. */
    export let shadow: CssClasses = "shadow";
    /** Provide classes to set the button styles. */
    export let button: CssClasses = "btn btn-sm variant-soft !text-white";

    // Base Classes
    const cBase = "overflow-scroll shadow";
    const cHeader = "text-xs text-white/50 uppercase p-2 pl-4";
    const cPre = "break-all p-4 pt-0 overflow-x-auto";
    const classesBase = `${cBase} ${background} ${blur} ${text} 
        ${color} ${rounded} ${shadow} ${$$props.class ?? ""}`;

    // Local
    let formatted = false;
    let displayCode: string = code;
    let copyState = false;
    let hover = false;

    // Allow shorthand alias, but show full text in UI
    function languageFormatter(lang: string): string {
        return (
            {
                js: "javascript",
                ts: "typescript",
                py: "python",
                sh: "bash",
                yml: "yaml",
                md: "markdown",
                rs: "rust",
                go: "golang",
                cpp: "C++",
                c: "C",
                cs: "C#",
                html: "HTML",
                css: "CSS",
            }?.[lang] ?? lang
        );
    }

    // Handle Copy Text
    function onCopyClick() {
        copyState = true;
        // prettier-ignore
        setTimeout(() => { copyState = false; }, 2000);
        /** @event {{}} copy - Fires when the Copy button is pressed.  */
        dispatch("copy", {});
    }

    // Trigger syntax highlighting if highlight.js is available
    $: if ($storeHighlightJs !== undefined) {
        displayCode = $storeHighlightJs
            .highlight(code, { language })
            .value.trim();
        formatted = true;
    }

    $: if (lineNumbers) {
        displayCode = displayCode.replace(/^/gm, () => {
            return '<span class="line"></span>\t';
        });
        formatted = true;
    }

    // Reactive
    $: txt = language == "plaintext" || language == "txt";
</script>

{#if language && code}
    <div
        class="codeblock relative {classesBase}"
        data-testid="codeblock"
        on:mouseenter={() => (hover = true)}
        on:mouseleave={() => (hover = false)}
    >
        <!-- Copy Button -->
        {#if copy && (hover || copyState)}
            <button
                class="codeblock-btn absolute right-1.5 top-1.5 {button}"
                on:click={onCopyClick}
                use:clipboard={code}
                transition:fade={{ duration: 100 }}
            >
                {#if copyState}
                    <span class="text-xs">Copied!</span><Check size={16} />
                {:else}
                    <Copy size={16} />
                {/if}
            </button>
        {/if}
        <!-- Header -->
        {#if !txt}
            <header class="codeblock-header {cHeader}">
                <!-- Language -->
                <span class="codeblock-language">
                    {languageFormatter(language)}
                </span>
            </header>
        {/if}
        <!-- Pre/Code -->
        <pre class="codeblock-pre {cPre}" class:mt-4={txt}><code
                class="codeblock-code language-{language} lineNumbers"
                >{#if formatted}{@html displayCode}{:else}{code.trim()}{/if}</code
            ></pre>
    </div>
{/if}
