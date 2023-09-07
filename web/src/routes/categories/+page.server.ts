import { getCategories, type CategoryConfigs } from "$lib/fetch";

class Data {
    categories: CategoryConfigs;
}

export const load = async ({ params }): Promise<Data> => {
    return {
        categories: await getCategories(),
    }
};