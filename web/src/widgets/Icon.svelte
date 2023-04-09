<script lang="ts">
    export let icon: string;
    export let color = "inherit";
    export let size = "1rem";
    export let reverse = false;

    $: style = `
        color: ${color};
        font-size: ${size};
    `;
</script>

{#if $$slots.default}
    <span class="flex" data-reverse={reverse}>
        <span class="material-symbols-sharp" {style}>
            {icon}
        </span>
        <span id="text" style:color>
            <slot />
        </span>
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
        flex-direction: row;

        &[data-reverse="true"] {
            flex-direction: row-reverse;
        }
    }

    .material-symbols-sharp {
        user-select: none;
        position: relative;
    }

    [data-reverse="true"] #text {
        margin-right: 0.25em;
    }

    [data-reverse="false"] #text {
        margin-left: 0.25em;
    }
</style>
