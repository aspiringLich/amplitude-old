import { getCategories, type CategoryConfigs, getProblemCompletion } from "$lib/fetch";

class Data {
    categories: CategoryConfigs;
    completed: Map<string, string[]>;
    incomplete: Map<string, string[]>;
}

export const load = async ({ params }): Promise<Data> => {
    let categories = await getCategories();
    let completion = await getProblemCompletion();
    
    return {
        categories,
        completed: completion.completed,
        incomplete: completion.incomplete,
    }
};