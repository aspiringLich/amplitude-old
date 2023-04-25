<script lang="ts">
    export let articles: ChildEntry[];

    import type { ChildEntry } from "./article";
</script>

<ul>
    {#each articles as article}
        <li>
            {article.title}
        </li>
    {/each}
</ul>

<style lang="scss">
    ul {
        left: 5px;

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
