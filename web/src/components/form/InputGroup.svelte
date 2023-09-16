<script lang="ts">
    import FormElement from "$cmpt/form/FormElement.svelte";

    /** The Title for the Element */
    export let title: string;

    export let values: string[];

    /** The Placeholder Text to display in the input boxes */
    export let placeholders: string[] = [];
</script>

<FormElement bind:title class="flex">
    <svelte:fragment slot="desc"><slot /></svelte:fragment>
    {#each values as value, i}
        <input class="input variant-form-material" class:ml-2={i != 0} bind:value placeholder={placeholders[i]} {...$$restProps} />
    {/each}
</FormElement>
