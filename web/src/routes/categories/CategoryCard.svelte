<script lang="ts">
    import { ProgressRadial } from "@skeletonlabs/skeleton";

    export let config;
</script>

<div class="card max-w-lg card-hover">
    <section class="p-4 flex gap-x-4">
        <div>
            <ProgressRadial width="w-[3.25rem]" value={0.4} />
        </div>
        <div>
            <h2 class="text-xl font-bold text-ellipsis whitespace-nowrap overflow-hidden">
                {config.title}
            </h2>
            <i class="font-light text-ellipsis whitespace-nowrap overflow-hidden">
                {config.description}
            </i>
        </div>
    </section>
</div>
