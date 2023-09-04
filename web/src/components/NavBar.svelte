<script lang="ts" context="module">
    type Path = [string, string][];
    /**
     * The path for the breadcrumbs.
     */
    export const path = writable([] as Path);
</script>

<script lang="ts">
    import { browser } from "$app/environment";
    import {
        LightSwitch,
        Avatar,
        getModalStore,
        getDrawerStore,
    } from "@skeletonlabs/skeleton";
    import { HamburgerMenu, TriangleDown } from "radix-icons-svelte";
    import { writable } from "svelte/store";

    export let pathname: string;

    const modalStore = getModalStore();
    const drawerStore = getDrawerStore();

    $: console.log($path);
    // export let path: string;
    // $: list = path.split("/").slice(1, path.length - 1);
    // $: trunc = list.slice(0, list.length - 1);

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
    <div class="flex items-center">
        <button
            class="hover-highlight interactive p-1 rounded-full ml-4"
            on:click={() =>
                drawerStore.open({
                    width: "w-96",
                })}
        >
            <HamburgerMenu size={20} />
        </button>
        <ol class="breadcrumb ml-4">
            <li class="crumb">
                <a class="anchor clean-link" href="/">home</a>
            </li>

            {#each $path as p, i}
                <li class="crumb-seperator" aria-hidden>/</li>
                <li class="crumb">
                    <a class="anchor clean-link" href={p[0]}>
                        {p[1]}
                    </a>
                </li>
            {/each}

            <li class="crumb-seperator" aria-hidden>/</li>
            <li class="crumb">
                <a class="anchor clean-link" href={pathname}>
                    {pathname.split("/").slice(-1)[0]}
                </a>
            </li>
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
