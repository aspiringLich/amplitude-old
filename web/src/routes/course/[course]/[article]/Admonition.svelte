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
    onMount(() => {
        if (body_element.children.length == 1) {
            let child = body_element?.firstElementChild;
        }
    });

    let col = "";
    // $: col = types[type].color;
    // console.log("e");
</script>

<div class="admonition">
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
    <blockquote class="body" bind:this={body_element}>
        <slot />
    </blockquote>
</div>

<style lang="scss">
    @use "@styles/variables";

    .admonition {
        // border: 1px solid;
        // border-radius: 0.3em;
        margin: 1em 0;
        // padding: 0.5em;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;

        #title {
            height: 2.5em;
            width: 100%;
            border-radius: variables.$border-radius variables.$border-radius 0 0;
            padding: 0.5em;
            box-sizing: border-box;
        }
    }
</style>
