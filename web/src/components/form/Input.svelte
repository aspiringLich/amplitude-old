<script lang="ts">
    import FormElement from "$cmpt/form/FormElement.svelte";

    /** The Title for the Element */
    export let title: string;

    export let value: string;

    /** The Placeholder Text to display in the input box */
    export let placeholder: string = "";
</script>

<FormElement bind:title>
    <svelte:fragment slot="desc"><slot /></svelte:fragment>
    <input class="input variant-form-material" bind:value {placeholder} {...$$restProps} />
</FormElement>
