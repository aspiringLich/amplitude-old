import type { EntryGenerator, RouteParams } from "./$types";
import { fetchApi, fetchItemList } from "$lib/fetch";
import type { Item } from "./item";

export const load = async ({ params, fetch }): Promise<Item> => {
    let response: Item = await fetchApi("/api/item", {
        method: "POST",
        body: {
            id: `${params.course}/${params.item}`,
        },
        fetch,
    });
    // renderComponent(doc.body, "pre", (await import(`./Code.svelte`)).default);
    // renderComponent(doc.body, "admonition", (await import(`./Admonition.svelte`)).default);

    return response;
};

export const entries = (async () => {
    let list = await fetchItemList();
    return list
        .map((item) => item.split["/"])
        .map((item) => {
            return { course: item[0], item: item[1] };
        });
}) satisfies EntryGenerator;

export const prerender = true;
// export const csr = false;
