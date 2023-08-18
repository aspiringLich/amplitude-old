import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi, getItemList } from "$lib/fetch";
import type { ArticleData, Item } from "$lib/item";

export const load = async ({ params, fetch }): Promise<Item> => {
    let response: Item = await fetchApi("/api/item", {
        method: "POST",
        body: {
            id: `${params.course}/${params.item}`,
        },
        fetch,
    });
    
    if (response.type == "article") {
        let inject_data = (response as any).inject_data;
        (response as any).inject_data = undefined;
        
        response.quiz_data = {}
        let promises = inject_data.quiz.map((quiz) => {
            return fetchApi("/api/item", {
                method: "POST",
                body: {
                    id: `${params.course}/${params.item}/${quiz.id}`,
                },
                fetch,
            });
        });
        
        for (const promise of promises) {
            let quiz = await promise;
            response.quiz_data[quiz.id] = quiz;
        }
    }

    return response;
};

export const entries = (async () => {
    let list = await getItemList();
    return list
        .map((item) => item.split("/"))
        .map((item) => {
            return { course: item[0], item: item[1] };
        });
}) satisfies EntryGenerator;

export const prerender = true;
// export const csr = false;
