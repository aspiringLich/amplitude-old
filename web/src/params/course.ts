import { getItemList } from "$lib/fetch";
import type { ParamMatcher } from "@sveltejs/kit";

const courses = new Set((await getItemList()).map((item) => item.split("/")[0]));

export const match = ((param) => {
    return courses.has(param);
}) satisfies ParamMatcher;
