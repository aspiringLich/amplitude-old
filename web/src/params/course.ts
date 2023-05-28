import { fetchItemList } from "$lib/fetch";
import type { ParamMatcher } from "@sveltejs/kit";

const courses = new Set((await fetchItemList()).map((item) => item.split("/")[0]));

export const match = ((param) => {
    return courses.has(param);
}) satisfies ParamMatcher;
