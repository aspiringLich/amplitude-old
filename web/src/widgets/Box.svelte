<script>
    export let text;
    export let hue;
</script>

<div id="box">
    <slot />
</div>

<style lang="scss">
    #box {
        background-color: hsl(var(--hue), 100%, 95%);
        border-radius: 4px;
        padding: 16px;
        color: var(--text);
        width: 100%;
    }
</style>
