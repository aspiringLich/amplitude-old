import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi } from "$lib/fetch";
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

// export const prerender = true;
// export const csr = false;
