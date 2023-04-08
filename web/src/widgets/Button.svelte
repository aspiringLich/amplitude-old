<script lang="ts">
    export let color: string;
    export let onclick = () => {};
    export let enabled = true;
    export let grayout = true;
    // 0 - 8px 16px
    // 1 - 4px
    // 2 - 0px
    export let padding = 0;
    export let stretch = false;

    let padding_style = {
        0: "8px 16px",
        1: "4px",
        2: "0px",
    }[padding];

    $: col = enabled || !grayout ? color : "1";
</script>

<div
    style:--click="var(--color-{col}-l0)"
    style:--hover="var(--color-{col}-l2)"
    style:--background="var(--color-{col}-l1)"
    style:--dark="var(--color-{col}-d1)"
    style:padding={padding_style}
    class:enabled
    class:stretch
    on:click={() => enabled && onclick()}
    on:keydown={(event) => event.key == "Enter" && onclick}
>
    <slot />
</div>

<style lang="scss">
    @use "../utils.scss" as *;

    div {
        position: inherit;
        display: inline-block;
        line-height: 100%;

        &.stretch {
            box-sizing: border-box;
            width: 100%;
            height: 100%;
        }

        text-decoration: none;
        border-radius: 4px;
        border: 1.5px solid var(--dark);
        color: var(--dark);
        user-select: none;
        // transition: 0.15s;
        background: gradient(var(--background), 40%);

        &:hover.enabled {
            background: gradient(var(--hover), 40%);
        }

        &:active.enabled {
            background: gradient(var(--click), 40%);
        }
    }
</style>
