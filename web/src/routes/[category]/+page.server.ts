import type { EntryGenerator, RouteParams } from "./$types";
import { CategoryConfig, fetchApi, getExerciseList, type List } from "$lib/fetch";

class Data {
    course: CategoryConfig;
    list: List;
}

export const load = async ({ params, fetch }): Promise<Data> => {
    let response: CategoryConfig = await fetchApi("/api/category", {
        method: "POST",
        body: {
            category: `${params.category}`,
        },
        fetch,
    });

    return {
        course: response,
        list: (await fetchApi("/api/list"))[params.category],
    };
};

// export const prerender = true;
// export const csr = false;
