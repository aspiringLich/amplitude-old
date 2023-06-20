import { localStorageStore } from "@skeletonlabs/skeleton";
import type { Writable } from "svelte/store";

export type EditorSettings = {
    theme: string;
    fontSize: number;
    flipPanes: boolean;
};

export const editorSettings: Writable<EditorSettings> = localStorageStore(
    "localEditorSettings",
    {
        theme: "tomorrow",
        fontSize: 14,
        flipPanes: false,
    }
);