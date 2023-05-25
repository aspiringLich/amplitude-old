<script lang="ts">
    export let color: string = "green";
    export let enabled = true;
    export let padding = 1.0;
    export let onclick = () => {};

    const hpad = 1.25;
    const vpad = 0.75;   
</script>

<button
    style:--local-background="var(--{color}-800)"
    style:--local-click="var(--{color}-900)"
    style:--local-color="var(--{color})"
    style:padding={`${padding * vpad}em ${padding * hpad}em`}
    class:enabled
    on:click={onclick}
    {...$$restProps}
>
    <div class="flex">
        <slot />
    </div>
</button>

<style lang="scss">
    .flex {
        display: flex;
        color: inherit;
        gap: var(--gap);
    }

    button {
        appearance: none;
        border: none;

        position: inherit;
        display: inline-block;
        line-height: 100%;

        text-decoration: none;
        border-radius: 4px;
        user-select: none;

        font-weight: 700;

        transition: color 0.1s linear, background-color 0.1s linear;

        :global(path) {
            stroke-width: 0.06em;
        }
    }

    button.enabled {
        cursor: pointer;

        background: var(--local-background);
        color: var(--local-color);
        &:active {
            background: var(--local-click);
        }
    
        :global(path) {
            stroke: var(--local-color);
        }
    }

    button:not(.enabled) {
        background-color: var(--gray-600);
        color: var(--gray);
    }
</style>
