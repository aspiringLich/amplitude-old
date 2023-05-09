<script lang="ts">
    export let entries: any;
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
