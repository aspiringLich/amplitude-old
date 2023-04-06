<script lang="ts">
    export let text = "black";
    export let id = "box";
    export let shadow = "0";
    export let hue = 0;
    export let sat = 0;
    export let val = 100;

    let color = `hsl(${hue}, ${sat}%, ${val}%)`;
    let border = `hsl(${hue}, ${sat}%, ${val * 0.9}%)`;
</script>

<div
    {id}
    style:box-shadow={shadow != "0" ? `0 0 ${shadow} rgba(0, 0, 0, 0.2)` : "none"}
    style:color={text}
    style:background={color}
    style:border={`1px solid ${border}`}
>
    <slot />
</div>

<style lang="scss">
    div {
        border-radius: 4px;
        padding: 8px 16px;
        margin: 16px 0px;
    }
</style>
