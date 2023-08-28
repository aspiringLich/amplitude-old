import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi, getExerciseList } from "$lib/fetch";
import type { ArticleData, Item } from "$lib/item";

export const load = async ({ params, fetch }): Promise<Item> => {
    let response: Item = await fetchApi("/api/exercise", {
        method: "POST",
        body: {
            id: `${params.category}/${params.exercise}`,
        },
        fetch,
    });

    return response;
};

export const entries = (async () => {
    let list = await getExerciseList();
    return list
        .map((item) => item.split("/"))
        .map((item) => {
            return { category: item[0], exercise: item[1] };
        });
}) satisfies EntryGenerator;

// export const prerender = true;
// export const csr = false;
