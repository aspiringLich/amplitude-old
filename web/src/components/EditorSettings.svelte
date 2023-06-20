<script lang="ts">
    import { modalStore } from "@skeletonlabs/skeleton";
    import { editorSettings as settings } from "$lib/settings";
    import * as themes from "thememirror";
    import Settings from "$cmpt/Settings.svelte";
    import Editor from "$cmpt/Editor.svelte";
    import Checkbox from "$cmpt/form/Checkbox.svelte";
    import StepSelect from "$cmpt/form/StepSelect.svelte";

    // https://stackoverflow.com/questions/7225407/convert-camelcasetext-to-title-case-text
    const titleize = (str: string) => {
        return str
            .replace(/([A-Z])/g, (match) => ` ${match}`)
            .replace(/^./, (match) => match.toUpperCase())
            .trim();
    };
    let trimmed_themes = [
        "default",
        ...Object.keys(themes).filter((x) => x != "createTheme"),
    ];
    let value = `import itertools, random\n\ndeck = list(itertools.product(
    range(1,14),\n    ['Spade','Heart','Diamond','Club']\n))random.shuffle(deck)
\nprint("You got:")\nfor i in range(5):\n   print(deck[i][0], "of", deck[i][1])`;
</script>

{#if $modalStore[0]}
    <Settings title="Editor">
        <Checkbox title="Flip Panes" bind:checked={$settings.flipPanes} />
        <Editor
            class="w-full h-64 min-h-64 max-h-64"
            lang_name="python"
            bind:value
        />
        <StepSelect
            title="Editor Theme"
            options={trimmed_themes}
            transform={titleize}
            bind:value={$settings.theme}
        />
    </Settings>
{/if}
