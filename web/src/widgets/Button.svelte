<script lang="ts">
    export let color: string;
    export let onclick = () => {};
    export let enabled = true;
    export let small = "false";

    $: col = enabled ? color : "1";
</script>

<div
    style:--click="var(--color-{col}-l0)"
    style:--hover="var(--color-{col}-l2)"
    style:--background="var(--color-{col}-l1)"
    style:--dark="var(--color-{col}-d1)"
    data-small={small}
    on:click={() => enabled && onclick()}
    on:keydown={(event) => event.key == "Enter" && onclick}
>
    <slot />
</div>

<style lang="scss">
    div {
        position: inherit;
        display: inline-block;
        line-height: 100%;

        &[data-small="false"] {
            padding: 8px 16px;
        }
        
        &[data-small="true"] {
            padding: 4px;
        }

        text-decoration: none;
        border-radius: 4px;
        border: 1px solid var(--dark);
        color: var(--dark);
        user-select: none;
        // transition: 0.15s;
        background: var(--background);

        &:hover {
            background: var(--hover);
        }

        &:active {
            background: var(--click);
        }
    }
</style>
