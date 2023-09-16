import { browser } from "$app/environment";
import { itemID } from "$lib/item";

export const fetchApi = async <T>(
    url: string,
    opts?: {
        method?: "POST" | "GET";
        body?: any;
        fetch?: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;
    }
): Promise<T> => {
    opts = opts ?? {};
    opts.fetch = opts.fetch ?? fetch;
    opts.method = opts.method ?? "GET";

    let input = browser ? url : `http://127.0.0.1:8080${url}`;
    const req = await opts.fetch(input, {
        method: opts.method ?? "GET",
        headers: {
            "Content-Type": "application/json",
        },
        body: opts.method == "POST" ? JSON.stringify(opts.body) : undefined,
    });
    if (!req.ok) {
        console.error(req);
        throw new Error(`failed to fetch ${url} with` + JSON.stringify(opts));
    }

    return await req.json();
};

// export type List = { [key: string]: string[] };

// export const getExerciseList = async (): Promise<string[]> => {
//     let list: List = await fetchApi("/api/list", { method: "GET" });
//     let items: string[] = [];
//     for (const [_, value] of Object.entries(list)) {
//         for (const item of value) {
//             items.push(...item);
//         }
//     }
//     return items;
// };

export type TestResult =
    | {
          type: "correct";
          stdout: string;
      }
    | {
          type: "incorrect";
          stdout: string;
          output: Object;
      }
    | {
          type: "error";
          stdout: string;
          traceback: string;
      };

export class TestResults {
    [key: string]: {
        results: TestResult[];
        hidden: boolean;
        passed: boolean;
    };
}

export class CategoryConfig {
    title: string;
    description: string;
    exercises: string[];
}

export type CategoryConfigs = { [key: string]: CategoryConfig };

export const getCategories = async (): Promise<CategoryConfigs> => {
    return await fetchApi("/api/list", { method: "GET" });
};

export type ProblemIds = { [key: string]: string[] };
export type ProblemCompletion = {
    completed: ProblemIds;
    incomplete: ProblemIds;
};

export const getProblemCompletion = async (): Promise<ProblemCompletion> => {
    return await fetchApi("/api/problem/completion", { method: "GET" });
};

export class CompletionConfig extends CategoryConfig {
    completed: string[];
    incomplete: string[];
}
export type CompletionConfigs = { [key: string]: CompletionConfig };

export const getCompletionConfig = async (): Promise<CompletionConfigs> => {
    let categories = await getCategories();
    let completion = await getProblemCompletion();

    let completion_config: { [key: string]: CompletionConfig } = {};
    for (const [category, config] of Object.entries(categories)) {
        let completed = completion.completed[category] ?? [];
        let incomplete = completion.incomplete[category] ?? [];
        completion_config[category] = {
            ...config,
            completed: completed,
            incomplete: incomplete,
        };
    }

    return completion_config;
};

export class ExerciseConfig {
    title: string;
    instructions: string;
    functions?: {
        [key: string]: {
            inputs: string[];
            output: string;
            hidden_cases: number;
            visible_cases: number;
            tests: {
                inputs: Object[];
                output: Object;
            }[];
        };
    };
}

export class ExerciseData {
    config: ExerciseConfig;
    lang_info: {
        [key: string]: {
            code: string;
        };
    };
    type?: "exercise";
}

export class LoginProvider {
    name: string;
    path: string;
}

export class Session {
    platform: string;
    token: string;
    id: string;
    name: string;
    avatar: string;
    signup: number;
    admin: boolean;
}

// todo: top level await makes build sad because old browsers don't support it
export const supportedPlatforms: [LoginProvider] = await fetchApi("/auth/supported", { method: "GET" });

export const getExercise = async (id: string): Promise<ExerciseData> => {
    return await fetchApi("/api/exercise", {
        method: "POST",
        body: {
            id: id,
        },
    });
};

export const getCategoryExercises = async (category: string): Promise<{ [key: string]: ExerciseConfig }> => {
    return await fetchApi("/api/exercise/category", {
        method: "POST",
        body: {
            category: category,
        },
    });
};
