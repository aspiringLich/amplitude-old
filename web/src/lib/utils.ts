import { browser } from "$app/environment";

export const fetchApi = async <T>(
    url: string,
    opts?: {
        method?: "POST" | "GET",
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
    
    // console.log("fetching", url, opts, "\n")
    let input = browser ? url : `http://127.0.0.1:8080${url}`;
    const req = await opts.fetch(input, {
        method: opts.method ?? "GET",
        headers: {
            "Content-Type": "application/json",
        },
        body: opts.method == "POST" ? JSON.stringify(opts.body) : undefined,
    });
    if (!req.ok) {
        console.log(req);
        throw new Error(`failed to fetch ${url} with` + JSON.stringify(opts));
    }

    return await req.json();
};
