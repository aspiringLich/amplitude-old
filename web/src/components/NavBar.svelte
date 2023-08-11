<script lang="ts">
    import { browser } from "$app/environment";
    import { LightSwitch, Avatar, modalStore } from "@skeletonlabs/skeleton";
    import { TriangleDown } from "radix-icons-svelte";

    export let path: string;
    $: list = path.split("/").slice(1, path.length - 1);
    $: trunc = list.slice(0, list.length - 1);

    const login = () => {
        modalStore.trigger({
            type: "component",
            component: "Login",
        });
    };
</script>

<div
    class="grid grid-cols-[1fr_1fr] grid-flow-row justify-between
    bg-surface-50-900-token pt-1 pb-1.5"
>
    <div>
        <ol class="breadcrumb ml-4">
            <li class="crumb"><a class="anchor" href="/">home</a></li>

            {#each trunc as crumb, i}
                <li class="crumb-seperator" aria-hidden>/</li>
                <li class="crumb">
                    <a class="anchor" href="/{trunc.slice(i).join('/')}">
                        {crumb}
                    </a>
                </li>
            {/each}

            <li class="crumb-seperator" aria-hidden>/</li>
            <li class="crumb">{list[list.length - 1]}</li>
        </ol>
    </div>
    <div class="flex items-center justify-end gap-2 px-2">
        <button
            class="select-none font-normal hover:underline"
            on:click={login}
        >
            Not Logged In<TriangleDown class="inline-block" size={18} />
        </button>
        <LightSwitch height="h-5" width="w-10" class="my-1" />
    </div>
</div>
