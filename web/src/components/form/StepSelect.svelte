<script lang="ts">
    import { ArrowLeft, ArrowRight } from "radix-icons-svelte";

    //~ Props
    export let value: any;
    /** The Title of the Element */
    export let title: string;
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

<label class="label block">
    <span>{title}</span>
    <div class="btn-group block">
        <div class="inline-flex">
            <button on:click={inc}><ArrowLeft /></button>
            <select class="select" bind:value={selected}>
                {#each options as option, i}
                    <option value={i}>{transform(option)}</option>
                {/each}
            </select>
            <button on:click={dec} class="border-l-0">
                <ArrowRight />
            </button>
        </div>
    </div>
</label>
