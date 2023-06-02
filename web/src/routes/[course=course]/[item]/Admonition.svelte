<script lang="ts">
    import { onMount } from "svelte";
    import {
        File,
        InfoCircled,
        ExclamationTriangle,
        CheckCircled,
        CrossCircled,
    } from "radix-icons-svelte";

    export let type: string;
    export let darken: number = 0;

    const data = {
        note: { color: "secondary", icon: File },
        info: { color: "primary", icon: InfoCircled },
        warning: { color: "warning", icon: ExclamationTriangle },
        success: { color: "success", icon: CheckCircled },
        correct: { color: "success", icon: CheckCircled },
        failure: { color: "error", icon: CrossCircled },
        incorrect: { color: "error", icon: CrossCircled },
    };
    let item = data[type];
    if (!item) throw new Error(`Unknown admonition type: ${type}`);
    let capitalized = type[0].toUpperCase() + type.slice(1);

    let body_element: HTMLElement;
    onMount(() => {
        if (body_element.children.length == 1) {
            let child = body_element?.firstElementChild;
        }
    });

    // $: col = types[type].color;
    // console.log("e");
</script>

<div class="admonition">
    <div
        class="title"
        style:background-color="var(--{item.color}-{8 - darken}00)"
        style:--local-color="var(--{item.color})"
    >
        <svelte:component this={item.icon} size={18} />
        {capitalized}
    </div>
    <blockquote class="body" bind:this={body_element}>
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

        .title {
            color: var(--local-color);

            height: 2.5em;
            width: 100%;
            border-radius: var(--border-radius) var(--border-radius) 0 0;
            padding: 0.5em;
            box-sizing: border-box;

            display: flex;
            gap: var(--gap);
            align-items: center;

            :global(path) {
                stroke: var(--local-color);
                stroke-width: 0.03em;
            }
        }
    }

    blockquote {
        border-top-left-radius: 0;
        border-top-right-radius: 0;
    }
</style>
