import type { EntryGenerator, RouteParams } from "./$types";
import { CourseConfig, fetchApi, getExerciseList, type List } from "$lib/fetch";

class Data {
    course: CourseConfig;
    list: List;
}

export const load = async ({ params, fetch }): Promise<Data> => {
    let response: CourseConfig = await fetchApi("/api/category", {
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
