import { localStorageStore } from "@skeletonlabs/skeleton";
import type { Writable } from "svelte/store";

export type EditorSettings = {
    theme: string | undefined;
    fontSize: number;
    flipPanes: boolean;
};

export const editorSettings: Writable<EditorSettings> = localStorageStore(
    "localEditorSettings",
    {
        theme: undefined,
        fontSize: 14,
        flipPanes: false,
    }
);