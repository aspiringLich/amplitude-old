<script lang="ts">
    export let icon;
    export let color = "inherit";
    export let size = "1rem";
    export let reverse = false;
    
    let flex = reverse ? "row-reverse" : "fl-row";
    $: style = `
        color: ${color};
        font-size: ${size};
    `;
</script>

{#if $$slots.default}
    <span class="flex" style:flex-direction={flex}>
        <span class="material-symbols-sharp" {style}>
            {icon}
        </span>
        <div style:color={color}>
            <slot />
        </div>
    </span>
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
    }

    .material-symbols-sharp {
        user-select: none;
        position: relative;
    }
</style>
