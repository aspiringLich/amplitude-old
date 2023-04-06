<script lang="ts">
    export let text: any;
    export let hue: number;
</script>

<div id="box" style="background-color: hsl({hue}, 100%, 95%);color:{text}">
    <slot />
</div>

<style lang="scss">
    #box {
        border-radius: 4px;
        padding: 16px;
        width: 100%;
    }
</style>
