import { localStorageStore } from "@skeletonlabs/skeleton";
import type { Writable } from "svelte/store";

export type EditorSettings = {
    lightTheme: string;
    darkTheme: string;
    fontSize: number;
    flipPanes: boolean;
};

export const editorSettings: Writable<EditorSettings> = localStorageStore(
    "localEditorSettings",
    {
        lightTheme: "tomorrow",
        darkTheme: "dracula",
        fontSize: 14,
        flipPanes: false,
    }
);