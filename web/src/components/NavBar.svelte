<script lang="ts">
    import { session } from "$lib/stores";
    import { LightSwitch, Avatar, getModalStore, getDrawerStore } from "@skeletonlabs/skeleton";
    import { HamburgerMenu, TriangleDown } from "radix-icons-svelte";

    type Path = [string, string][];

    export let path: Path = [];
    export let pathname: string;

    const modalStore = getModalStore();
    const drawerStore = getDrawerStore();

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
        <ol class="breadcrumb ml-4" tabindex="-1">
            <li class="crumb">
                <a class="anchor clean-link" href="/">home</a>
            </li>

            {#each path as p, i}
                <li class="crumb-seperator" aria-hidden>/</li>
                <li class="crumb">
                    <a class="anchor clean-link" href={p[0]}>
                        {p[1]}
                    </a>
                </li>
            {/each}

            {#if pathname !== "/"}
                <li class="crumb-seperator" aria-hidden>/</li>
                <li class="crumb">
                    <a class="anchor clean-link" href={pathname}>
                        {pathname.split("/").slice(-1)[0]}
                    </a>
                </li>
            {/if}
        </ol>
    </div>
    <div class="flex items-center justify-end gap-2 px-2">
        <button class="select-none font-normal hover:underline" on:click={login}>
            {#if $session}{$session.name}{:else}Not Logged In{/if}<TriangleDown class="inline-block" size={18} />
        </button>
        <LightSwitch height="h-5" width="w-10" class="my-1" />
    </div>
</div>
