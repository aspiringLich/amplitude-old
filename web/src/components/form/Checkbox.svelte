<script lang="ts">
    export let checked: boolean;
    
    export let title: string;
</script>

<div class="block">
    <label class="label inline-flex items-center">
        <input class="checkbox" type="checkbox" bind:checked />
        <span>{title}</span>
    </label>
</div>
