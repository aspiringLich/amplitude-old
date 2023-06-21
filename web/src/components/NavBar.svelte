<script lang="ts">
    import { browser } from "$app/environment";
    import { LightSwitch, Avatar } from "@skeletonlabs/skeleton";
    import { TriangleDown } from "radix-icons-svelte";
    
    let path = browser ? window.location.pathname.split("/").slice(1) : [];
    $: trunc = path.slice(0, path.length - 1);
</script>

<div
    class="grid grid-cols-[1fr_1fr_1fr] grid-flow-row justify-between
    bg-surface-50-900-token py-3"
>
    <div>
        <ol class="breadcrumb ml-4">
            <li class="crumb"><a class="anchor" href="/">root</a></li>
            
            {#each trunc as crumb, i}
                <li class="crumb-seperator" aria-hidden>/</li>
                <li class="crumb">
                    <a class="anchor" href="/{trunc.slice(i).join("/")}">
                        {crumb}
                    </a>
                </li>
            {/each}
            
            <li class="crumb-seperator" aria-hidden>/</li>
            <li class="crumb">{path[path.length - 1]}</li>
        </ol>
    </div>
    <div />
    <div class="flex items-center justify-end gap-2 px-2">
        <button class="select-none font-normal">
            Not Logged In<TriangleDown class="inline-block" size={18} />
        </button>
        <LightSwitch height="h-5" width="w-10" class="my-1" />
    </div>
</div>
