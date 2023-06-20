<script lang="ts">
    import {
        File,
        InfoCircled,
        ExclamationTriangle,
        CheckCircled,
        CrossCircled,
        Cross2,
    } from "radix-icons-svelte";
    import colors from "tailwindcss/colors";

    // Props
    /** The type of the admonition. Only accepts certain strings. */
    export let type:
        | "note"
        | "info"
        | "warning"
        | "success"
        | "correct"
        | "failiure"
        | "incorrect"
        | "error";
    /** Base classes for styling the outer div */
    export { classes as class };
    let classes: string = "";
    /** Base classes for styling the container itself */
    export let container = "";

    const title_color = 500;
    const body_color = 100;
    const data = {
        note: ["purple", File],
        info: ["cyan", InfoCircled],
        warning: ["amber", ExclamationTriangle],
        success: ["lime", CheckCircled],
        correct: ["lime", CheckCircled],
        failure: ["rose", CrossCircled],
        incorrect: ["rose", CrossCircled],
        error: ["rose", Cross2],
    };
    $: [color_type, icon] = data[type];
    $: title_text = type.at(0).toUpperCase() + type.slice(1);
</script>

<div class="admonition my-4 flex flex-col {classes}">
    <div
        class="title rounded-t w-full text-white font-extrabold flex items-center"
        style:background-color={colors[color_type][title_color]}
    >
        <svelte:component this={icon} size={18} class="m-1.5" />
        <span>{title_text}</span>
    </div>
    <blockquote
        class="rounded-t-none {container}"
        style:background-color={colors[color_type][body_color]}
    >
        <slot />
    </blockquote>
</div>
