<script lang="ts">
    export let color: string = "green";
    export let enabled = true;
    export let padding = 1.0;
    export let onclick = () => {};

    const hpad = 1.25;
    const vpad = 0.75;
</script>

<button
    style:--click="var(--{color}-light_)"
    style:--background="var(--{color}-light)"
    style:--color="var(--{color}-dark)"
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
        background: var(--background);
        color: var(--color);
        &:active {
            background: var(--click);
        }

        :global(path) {
            stroke: var(--color);
        }
    }

    button:not(.enabled) {
        cursor: default;
        background: var(--gray-light__);
        color: var(--gray-dark);

        :global(path) {
            stroke: var(--gray-dark);
        }
    }
</style>
