import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi } from "$lib/utils";

class ArticleResponse {
    body: string;
    name: string;
}

export const load = async ({ params, fetch }): Promise<ArticleResponse> => {
    let body = JSON.stringify({ article: params.article });
    console.log(body);
    return fetchApi("/api/article", {
        body,
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
