<script lang="ts">
    export let checked: boolean;
    
    export let title: string;
</script>

<div class="block">
    <label class="label inline-flex items-center">
        <input class="checkbox" type="checkbox" bind:checked />
        <span class="!mt-0 ml-2">{title}</span>
    </label>
</div>
