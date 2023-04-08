<script lang="ts">
    import { onMount } from "svelte";

    export let type: string;

    import Icon from "../widgets/Icon.svelte";

    let types = {
        note: { hue: 267 },
        info: { hue: 194 },
        warning: { hue: 53 },
        success: { hue: 95, icon: "check_circle" },
        failiure: { hue: 0, icon: "error" },
    };
    if (!(type in types)) throw new Error(`Unknown admonition type: ${type}`);
    let capitalized = type[0].toUpperCase() + type.slice(1);

    let border_color = `hsl(${types[type].hue}, 80%, 50%)`;
    let background_color = `hsl(${types[type].hue}, 100%, 95%)`;

    let body_element: HTMLElement;
    let l_padding = "1em";
    onMount(() => {
        if (body_element.children.length == 1) {
            let child = body_element.firstChild?.firstChild
                ?.firstChild as HTMLElement;
            console.log(child?.nodeName);
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
        <Icon type="inline" size="1.5em" color={border_color}>
            {types[type].icon ?? type}
        </Icon>
        <span style:color={border_color} id="title-span">
            {capitalized}
        </span>
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

        #title-span {
            font-weight: 900;
            position: relative;
            top: -0.35em;
        }
    }
</style>
