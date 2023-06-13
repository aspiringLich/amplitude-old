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
    /** Set the color of the title background */
    export let title_color = 500;
    /** Set the color of the body */
    export let body_color = 100;

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

    // Reactive

    // $: col = types[type].color;
    // console.log("e");
</script>

<div class="admonition my-4">
    <div
        class="title rounded-t w-full text-white font-extrabold flex items-center"
        style:background-color={colors[color_type][title_color]}
    >
        <svelte:component this={icon} size={18} class="m-1.5" />
        <span>{title_text}</span>
    </div>
    <blockquote
        class="body rounded-t-none"
        style:background-color={colors[color_type][body_color]}
    >
        <slot />
    </blockquote>
</div>

<style lang="scss">
    .admonition {
        margin: 1em 0;
        // padding: 0.5em;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;
    }
</style>
