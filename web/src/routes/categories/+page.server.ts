import { getCategories, type CategoryConfigs, getProblemCompletion, CategoryConfig } from "$lib/fetch";

class Data {
    categories: { [key: string]: CompletionConfig };
    completed: Map<string, string[]>;
    incomplete: Map<string, string[]>;
}

class CompletionConfig extends CategoryConfig {
    completed: number;
    incomplete: number;
}

export const load = async ({ params }): Promise<Data> => {
    let categories = await getCategories();
    let completion = await getProblemCompletion();

    let completion_config: { [key: string]: CompletionConfig } = {};

    for (const [key, value] of Object.entries(categories)) {
        let completed = completion.completed[key];
        let incomplete = completion.incomplete[key];
        completion_config[key] = {
            ...value,
            completed: completed ? completed.length : 0,
            incomplete: incomplete ? incomplete.length : 0,
        };
    }

    return {
        categories: completion_config,
        completed: completion.completed,
        incomplete: completion.incomplete,
    };
};
