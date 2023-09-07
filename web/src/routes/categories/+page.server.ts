import { getCategories, type CategoryConfigs } from "$lib/fetch";

export const load = async ({ params }): Promise<CategoryConfigs> => {
    return await getCategories();
};