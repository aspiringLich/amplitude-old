<script lang="ts">
    export let color: string = "green";
    export let onclick = () => {};
    export let enabled = true;
    export let grayout = true;
    // 0 - 8px 16px
    // 1 - 4px
    // 2 - 0px
    export let padding = 0;

    let padding_style = {
        0: "8px 16px",
        1: "4px",
        2: "0px",
    }[padding];

    $: col = enabled || !grayout ? color : "1";
</script>

<div
    class="button"
    style:--click="var(--{color}-light_)"
    style:--hover="var(--{color}-light-)"
    style:--background="var(--{color}-light)"
    style:color="var(--{color}-dark)"
    style:padding={padding_style}
    class:enabled
    on:click={() => enabled && onclick()}
    on:keydown={(event) => event.key == "Enter" && onclick}
>
    <div class="flex">
        <slot />
    </div>
</div>

<style lang="scss">
    .flex {
        display: flex;
        gap: var(--gap);
    }
    
    .button {
        position: inherit;
        display: inline-block;
        line-height: 100%;

        text-decoration: none;
        border-radius: 4px;
        user-select: none;
    }

    .button.enabled {
        background: var(--background);
        color: var(--dark);
        &:hover {
            background: var(--hover);
        }

        &:active {
            background: var(--click);
        }
    }

    .button:not(.enabled) {
        background: var(--gray-light);
        color: var(--gray-dark);
    }
</style>
