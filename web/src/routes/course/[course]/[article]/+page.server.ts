import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi } from "@src/lib/utils";
import { JSDOM } from "jsdom";

class ArticleResponse {
    body: string;
    config: {
        title: string;
        id: string;
    };
}

export const load = async ({ params, fetch }): Promise<ArticleResponse> => {
    let response: ArticleResponse = await fetchApi("/api/article", {
        method: "POST",
        body: params,
        fetch,
    });
    // renderComponent(doc.body, "pre", (await import(`./Code.svelte`)).default);
    // renderComponent(doc.body, "admonition", (await import(`./Admonition.svelte`)).default);

    return response;
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
// export const csr = false;
