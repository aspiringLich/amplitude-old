<script lang="ts">
    import { modalStore } from "@skeletonlabs/skeleton";
    import { editorSettings as settings } from "$lib/settings";
    import * as themes from "thememirror";
    import Modal from "$cmpt/modals/Modal.svelte";
    import Editor from "$cmpt/Editor.svelte";
    import Checkbox from "$cmpt/form/Checkbox.svelte";
    import StepSelect from "$cmpt/form/StepSelect.svelte";
    import { camelToTitle } from "$lib/util";

    const predicate = (x: string) => x != "createTheme";
    let trimmed_themes = ["default", ...Object.keys(themes)].filter(predicate);
    let value = `import itertools, random\n\ndeck = list(itertools.product(
    range(1,14),\n    ['Spade','Heart','Diamond','Club']\n))\nrandom.shuffle(deck)
\nprint("You got:")\nfor i in range(5):\n   print(deck[i][0], "of", deck[i][1])`;
</script>

{#if $modalStore[0]}
    <Modal>
        <h3 class="h3 !my-1">Editor Settings</h3>
        <div class="grid grid-cols-1 gap-2 !mt-0">
            <Editor
                class="w-full h-64 min-h-64 max-h-64"
                lang_name="python"
                bind:value
            />
            <StepSelect
                title="Editor Theme (light)"
                options={trimmed_themes}
                transform={camelToTitle}
                bind:value={$settings.lightTheme}
            />
            <StepSelect
                title="Editor Theme (dark)"
                options={trimmed_themes}
                transform={camelToTitle}
                bind:value={$settings.darkTheme}
            />
            <Checkbox title="Flip Panes" bind:checked={$settings.flipPanes} />
        </div>
    </Modal>
{/if}
