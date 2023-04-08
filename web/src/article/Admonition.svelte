<script lang="ts">
    import { onMount } from "svelte";

    export let type: string;

    import Icon from "../widgets/Icon.svelte";

    let types = {
        note: { color: "purple" },
        info: { color: "blue" },
        warning: { color: "yellow" },
        success: { color: "green", icon: "check_circle" },
        failiure: { color: "red", icon: "error" },
    };
    if (!(type in types)) throw new Error(`Unknown admonition type: ${type}`);
    let capitalized = type[0].toUpperCase() + type.slice(1);

    let border_color = `var(--color-${types[type].color}-d1)`;
    let background_color = `var(--color-${types[type].color}-l1)`;

    let body_element: HTMLElement;
    let l_padding = "1em";
    onMount(() => {
        if (body_element.children.length == 1) {
            let child = body_element.firstChild?.firstChild
                ?.firstChild as HTMLElement;
            if (child != undefined && child.nodeName == "CODE") {
                l_padding = "0";
            }
        }
    });
</script>

<div
    class={`admonition`}
    style:border-color={border_color}
    style:--l-padding={l_padding}
>
    <div id="title" style:background-color={background_color}>
        <Icon icon={types[type].icon ?? type} size="1.5em" color={border_color}>
            {capitalized}
        </Icon>
    </div>
    <div id="body" class="n-top-border-radius" bind:this={body_element}>
        <slot />
    </div>
</div>

<style lang="scss">
    .admonition {
        border: 1px solid;
        border-radius: 0.3em;
        margin: 1em 0;
        // padding: 0.5em;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        border: 1.5px solid;

        #body {
            padding-left: var(--l-padding);
            box-sizing: border-box;
            :global(p) {
                margin: 0.6rem 0;
            }
            width: 100%;
            overflow: hidden;
        }

        #title {
            height: 2.5em;
            border-radius: 0.3em;
            width: 100%;
            padding: 0.5em;
            box-sizing: border-box;
        }
    }
</style>
