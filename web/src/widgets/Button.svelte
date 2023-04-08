<script lang="ts">
    export let color: string;
    export let onclick = () => {};
    export let disabled = false;

    $: col = disabled ? "3" : color;
</script>

<div
    style:--click="var(--color-{col}-l0)"
    style:--hover="var(--color-{col}-l2)"
    style:--background="var(--color-{col}-l1)"
    style:--dark="var(--color-{col}-d1)"
    on:click={() => !disabled && onclick()}
    on:keydown={(event) => event.key == "Enter" && onclick}
>
    <slot />
</div>

<style lang="scss">
    div {
        display: inline-block;
        padding: 8px 16px;
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
