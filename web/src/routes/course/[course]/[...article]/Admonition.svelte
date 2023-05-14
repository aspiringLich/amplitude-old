<script lang="ts">
    import { onMount } from "svelte";

    export let type: string;

    let types = {
        note: { color: "purple" },
        info: { color: "blue" },
        warning: { color: "yellow" },
        success: { color: "green", icon: "check_circle" },
        failiure: { color: "red", icon: "error" },
    };
    if (!(type in types)) throw new Error(`Unknown admonition type: ${type}`);
    let capitalized = type[0].toUpperCase() + type.slice(1);

    let body_element: HTMLElement;
    let l_padding = "1em";
    onMount(() => {
        if (body_element.children.length == 1) {
            let child = body_element?.firstElementChild;
            if (child?.id == "code") l_padding = "0";
        }
    });
        
    let col = "";
    // $: col = types[type].color;
    // console.log("e");
</script>

<div class="admonition" style:--l-padding={l_padding}>
    <div id="title" style:background-color="var(--color-{col}-l1">
        {capitalized}
        <!-- <Icon
            icon={types[type].icon ?? type}
            size="1.5em"
            color="var(--color-{col}-d2)"
        >
            {capitalized}
        </Icon> -->
    </div>
    <div id="body" class="n-top-border-radius" bind:this={body_element}>
        <slot />
    </div>
</div>

<style lang="scss">
    $border-radius: 0.3em;

    .admonition {
        // border: 1px solid;
        // border-radius: 0.3em;
        margin: 1em 0;
        // padding: 0.5em;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;

        #body {
            padding-left: var(--l-padding);
            padding-top: 0.5em;
            padding-bottom: 0.5em;
            padding-right: 1.5em;
            box-sizing: border-box;
            :global(p) {
                margin: 0.6rem 0;
            }
            border-radius: 0 0 $border-radius $border-radius;
            width: 100%;
            overflow: hidden;
            background-color: #00000007;
        }

        #title {
            height: 2.5em;
            width: 100%;
            border-radius: $border-radius $border-radius 0 0;
            padding: 0.5em;
            box-sizing: border-box;
        }
    }
</style>
