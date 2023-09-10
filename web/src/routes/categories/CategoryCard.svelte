<script lang="ts">
    import type { CategoryConfig } from "$lib/fetch";

    export let config: CategoryConfig;
    export let id: string;
</script>

<div class="card min-w-96 max-w-lg">
    <section class="p-4">
        <h2 class="text-xl font-bold">{config.title}</h2>
        <i class="font-light">{config.description}</i>
    </section>
</div>
