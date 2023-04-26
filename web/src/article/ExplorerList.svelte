<script lang="ts">
    export let entries: ChildEntry[];

    import type { ChildEntry } from "./article";
</script>

<ul>
    {#each entries as entry}
        <li>
            {entry.title}
        </li>
    {/each}
</ul>

<style lang="scss">
    ul {
        padding-left: 10px;

        li {
            &[data-readable="true"] {
                color: black;
            }

            &[data-readable="false"] {
                color: var(--color-0-m);
            }
        }
    }
</style>
