<script lang="ts">
    export let type = "inline";
    export let args: any = {};
    export let background: string = undefined;
    export let color = "#000000";
    export let size = "1em";
</script>

<div
    class="material-symbols-sharp {type}"
    style:color
    style:font-size={size}
    style:background-color={background}
>
    {#if type == "inline"}
        <slot />
    {:else if type == "button"}
        <div on:click={args.onclick} on:keydown={args.onclick}>
            <slot />
        </div>
    {/if}
</div>

<style lang="scss">
    div {
        line-height: 100%;
        user-select: none;

        &.button {
            display: inline-block;
            cursor: pointer;
            // background: rgb(255, 255, 255);
            border-radius: 10%;
            line-height: 100%;
            padding: 5px;

            &:hover {
                filter: opacity(90%);
            }

            &:active {
                filter: opacity(50%);
            }
        }
    }
</style>
