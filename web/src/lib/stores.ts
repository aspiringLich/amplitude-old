import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import type { Session } from "./fetch";

export const session: Writable<Session | null> = writable(null);
