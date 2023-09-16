<script lang="ts">
    import FormElement from "$cmpt/form/FormElement.svelte";
    import { ArrowLeft, ArrowRight } from "radix-icons-svelte";

    //~ Props
    /** The Title for the Element */
    export let title: string;

    export let value: any;

    /** The list of options for the select */
    export let options: any[] = [];
    /** The function to run on the values when displayed */
    export let transform: (x: any) => string = (x) => x.toString();

    //~ State
    let selected = options.findIndex((x) => x === value);
    if (selected == -1) selected = 0;

    //~ Button Handlers
    const inc = () => {
        selected = (selected + 1) % options.length;
    };
    const dec = () => {
        selected = (selected - 1 + options.length) % options.length;
    };

    $: value = options[selected];
</script>

<FormElement bind:title>
    <svelte:fragment slot="desc"><slot /></svelte:fragment>
    <div class="btn-group bg-surface-200-700-token inline-flex mt-1">
        <button class="z-20" on:click={dec}><ArrowLeft size={18} /></button>
        <select class="select border-l-0 border-b-2 rounded-none" bind:value={selected}>
            {#each options as option, i}
                <option value={i}>{transform(option)}</option>
            {/each}
        </select>
        <button on:click={inc} class="border-l-0">
            <ArrowRight size={18} />
        </button>
    </div>
</FormElement>
