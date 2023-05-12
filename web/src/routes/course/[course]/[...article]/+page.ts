import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi } from "$lib/utils";

class ArticleResponse {
    body: string;
    name: string;
}

export const load = async ({ params, fetch }): Promise<ArticleResponse> => {
    return fetchApi("/api/article", {
        body: { article: params.article },
        fetch,
    });
};

export const entries = (async () => {
    type CourseItem = Map<String, [String | CourseItem]>;

    let data: CourseItem = await fetchApi("/api/course-list");
    return Object.entries(data)
        .map(([key, value]) =>
            [...new Set(value.flat())].map((item) => {
                return { course: key, article: item } as RouteParams;
            })
        )
        .flat();
}) satisfies EntryGenerator;

export const prerender = true;
