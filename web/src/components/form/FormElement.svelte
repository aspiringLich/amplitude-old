<script lang="ts">
    /** The Title for the Element */
    export let title: string;

    let className = "";
    export { className as class };
</script>

<label class="block my-2">
    <span class="block font-semibold p">{title}</span>
    <span class="block text-sm text-surface-400-700-token"><slot name="desc" /></span>
    <div class={className}>
        <slot />
    </div>
</label>
