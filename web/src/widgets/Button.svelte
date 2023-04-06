<script lang="ts">
    export let hue = 0;
    export let sat = 0;
    export let val = 100;
    export let onclick = () => {};

    function hsl(valFactor: number) {
        return `hsl(${hue}, ${sat}%, ${val * valFactor}%)`;
    }
    let color = hsl(0.95);
    let border = hsl(0.9);
    let hover = hsl(0.97);
    let text = hsl(0.5);
</script>

<div
    class="button"
    style:border="1.5px solid {border}"
    style:--color={color}
    style:--hover={hover}
    style:--click={border}
    style:color={text}
    on:click={onclick}
    on:keydown="{event => event.key == 'Enter' && onclick}"
>
    <slot />
</div>

<style lang="scss">
    .button {
        display: inline-block;
        padding: 8px 16px;
        text-decoration: none;
        border-radius: 4px;
        user-select: none;
        transition: 0.15s;
        background: var(--color);

        &:hover {
            background: var(--hover);
        }

        &:active {
            background: var(--click);
        }
    }
</style>
