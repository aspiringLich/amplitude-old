<script lang="ts">
    export let checked: boolean;
    
    export let title: string;
</script>

<div class="block">
    <label class="label inline-flex items-center">
        <input class="checkbox !bg-surface-300 dark:!bg-surface-600" type="checkbox" bind:checked />
        <span class="!mt-0 ml-2 font-semibold text-surface-700-200-token">{title}</span>
    </label>
</div>
