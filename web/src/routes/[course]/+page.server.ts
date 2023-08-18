import type { EntryGenerator, RouteParams } from "./$types";
import { CourseConfig, fetchApi, getItemList, type List } from "$lib/fetch";

class Data {
    course: CourseConfig;
    list: List;
}

export const load = async ({ params, fetch }): Promise<Data> => {
    let response: CourseConfig = await fetchApi("/api/course", {
        method: "POST",
        body: {
            course: `${params.course}`,
        },
        fetch,
    });

    return {
        course: response,
        list: (await fetchApi("/api/list"))[params.course],
    };
};

export const prerender = true;
// export const csr = false;
