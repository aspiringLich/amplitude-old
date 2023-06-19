import { getItemList } from "$lib/fetch";
import type { ParamMatcher } from "@sveltejs/kit";

const courses = new Set((await getItemList()).map((item) => item.split("/")[0]));

export const match = ((param) => {
    if (courses) return courses.has(param);
    return Boolean(
        getItemList().then((x) => {
            return x.map((x) => x.split("/")[0]).find((x) => x === param);
        })
    );
}) satisfies ParamMatcher;
