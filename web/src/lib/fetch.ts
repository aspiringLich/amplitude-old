import { browser } from "$app/environment";
import { itemID } from "$lib/item";

export const fetchApi = async <T>(
    url: string,
    opts?: {
        method?: "POST" | "GET";
        body?: any;
        fetch?: (
            input: RequestInfo | URL,
            init?: RequestInit
        ) => Promise<Response>;
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

export type List = { [key: string]: string[] };

export const getExerciseList = async (): Promise<string[]> => {
    let list: List = await fetchApi("/api/list");
    let items: string[] = [];
    for (const [_, value] of Object.entries(list)) {
        for (const item of value) {
            items.push(...item);
        }
    }
    return items;
};

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

export class CourseConfig {
    title: string;
    description: string;
    icon: string;
    index: string;
}