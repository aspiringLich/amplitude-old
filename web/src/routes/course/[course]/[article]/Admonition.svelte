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

    let data = {
        note: { color: "purple", icon: File },
        info: { color: "blue", icon: InfoCircled },
        warning: { color: "yellow", icon: ExclamationTriangle },
        success: { color: "green", icon: CheckCircled },
        failiure: { color: "red", icon: CrossCircled },
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
        id="title"
        style:background-color="var(--{item.color}-light)"
        style:color="var(--{item.color}-medium)"
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

        #title {
            height: 2.5em;
            width: 100%;
            border-radius: 0.3em 0.3em 0 0;
            padding: 0.5em;
            box-sizing: border-box;
            display: flex;
            gap: 4px;
            align-items: center;
        }
    }
</style>
