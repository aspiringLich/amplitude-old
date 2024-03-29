<!-- taken from https://github.com/touchifyapp/svelte-codemirror-editor -->

<script lang="ts" context="module">
    export type ThemeSpec = Record<string, StyleSpec>;
    export type StyleSpec = {
        [propOrSelector: string]: string | number | StyleSpec | null;
    };
</script>

<script lang="ts">
    import { createEventDispatcher, onDestroy, onMount } from "svelte";
    import { modeCurrent } from "@skeletonlabs/skeleton";
    import { basicSetup } from "codemirror";
    import {
        EditorView,
        keymap,
        placeholder as placeholderExt,
    } from "@codemirror/view";
    import {
        EditorState,
        StateEffect,
        type Extension,
    } from "@codemirror/state";
    import { indentWithTab } from "@codemirror/commands";
    import { indentUnit, type LanguageSupport } from "@codemirror/language";
    import { debounce } from "$lib/util";

    import { editorSettings as settings } from "$lib/settings";
    import * as themes from "thememirror";

    import { python } from "@codemirror/lang-python";

    let classes = "";
    export { classes as class };
    export let value: string | null | undefined = "";

    export let basic = true;
    export let theme: Extension | null | undefined = undefined;
    export let lang_name: string | undefined = undefined;
    export let extensions: Extension[] = [];

    export let useTab = true;
    export let tabSize = 4;

    export let styles: ThemeSpec | null | undefined = undefined;
    export let lineWrapping = false;
    export let editable = true;
    export let readonly = false;
    export let placeholder: string | HTMLElement | null | undefined = undefined;

    const is_browser = typeof window !== "undefined";
    const dispatch = createEventDispatcher<{ change: string }>();

    let element: HTMLDivElement;
    let view: EditorView;

    let update_from_prop = false;
    let update_from_state = false;
    let first_config = true;
    let first_update = true;

    $: lang = {
        python: python(),
    }[lang_name];

    $: theme =
        themes[$modeCurrent ? $settings.lightTheme : $settings.darkTheme];

    $: state_extensions = [
        ...get_base_extensions(
            basic,
            useTab,
            tabSize,
            lineWrapping,
            placeholder,
            editable,
            readonly,
            lang
        ),
        ...get_theme(theme, styles),
        ...extensions,
    ];

    $: view && update(value);
    $: view && state_extensions && reconfigure();

    onMount(() => {
        view = create_editor_view();
        element.querySelector(".cm-content").setAttribute("tabindex", "-1");
    });
    onDestroy(() => view?.destroy());

    function create_editor_view(): EditorView {
        const on_change = debounce(handle_change, 300);

        return new EditorView({
            parent: element,
            state: create_editor_state(value),
            dispatch(transaction) {
                view.update([transaction]);

                if (!update_from_prop && transaction.docChanged) {
                    on_change();
                }
            },
        });
    }

    function reconfigure(): void {
        if (first_config) {
            first_config = false;
            return;
        }

        view.dispatch({
            effects: StateEffect.reconfigure.of(state_extensions),
        });
    }

    function update(value: string | null | undefined): void {
        if (first_update) {
            first_update = false;
            return;
        }

        if (update_from_state) {
            update_from_state = false;
            return;
        }

        update_from_prop = true;

        view.setState(create_editor_state(value));

        update_from_prop = false;
    }

    function handle_change(): void {
        const new_value = view.state.doc.toString();
        if (new_value === value) return;

        update_from_state = true;

        value = new_value;
        dispatch("change", value);
    }

    function create_editor_state(
        value: string | null | undefined
    ): EditorState {
        return EditorState.create({
            doc: value ?? undefined,
            extensions: state_extensions,
        });
    }

    function get_base_extensions(
        basic: boolean,
        useTab: boolean,
        tabSize: number,
        lineWrapping: boolean,
        placeholder: string | HTMLElement | null | undefined,
        editable: boolean,
        readonly: boolean,
        lang: LanguageSupport | null | undefined
    ): Extension[] {
        const extensions: Extension[] = [
            indentUnit.of(" ".repeat(tabSize)),
            EditorView.editable.of(editable),
            EditorState.readOnly.of(readonly),
        ];

        if (basic) extensions.push(basicSetup);
        if (useTab) extensions.push(keymap.of([indentWithTab]));
        if (placeholder) extensions.push(placeholderExt(placeholder));
        if (lang) extensions.push(lang);
        if (lineWrapping) extensions.push(EditorView.lineWrapping);

        return extensions;
    }

    function get_theme(
        theme: Extension | null | undefined,
        styles: ThemeSpec | null | undefined
    ): Extension[] {
        const extensions: Extension[] = [];
        if (styles) extensions.push(EditorView.theme(styles));
        if (theme) extensions.push(theme);
        return extensions;
    }
</script>

<div class="codemirror-wrapper {classes}" bind:this={element} />

<style>
    /*! purgecss start ignore */
    .codemirror-wrapper {
        height: 100%;
    }
    .codemirror-wrapper :global(.cm-focused) {
        outline: none;
    }

    .codemirror-wrapper :global(.cm-editor) {
        height: 100%;
        max-height: none;
    }

    .codemirror-wrapper :global(.cm-gutter) {
        user-select: none;
        border-right-width: 0;
    }
</style>
