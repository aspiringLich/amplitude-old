<script lang="ts">
    export let icon;
    export let color = "inherit";
    export let size = "1rem";

    $: style = `
        color: ${color};
        font-size: ${size};
    `;
</script>

{#if $$slots.default}
    <div class="flex">
        <span class="material-symbols-sharp" {style}>
            {icon}
        </span>
        <div style:color={color}>
            <slot />
        </div>
    </div>
{:else}
    <span class="material-symbols-sharp" {style}>
        {icon}
    </span>
{/if}

<style lang="scss">
    .flex {
        display: flex;
        align-content: center;
        align-items: center;
        flex-direction: row;
    }

    .material-symbols-sharp {
        user-select: none;
        position: relative;
    }
</style>
