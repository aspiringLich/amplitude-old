import { getCompletionConfig, type CompletionConfigs } from "$lib/fetch";

export const load = async ({ params }) => {
    return {
        categories: await getCompletionConfig()
    };
};
