import { browser } from "$app/environment";

export const fetchApi = async <T>(
    url: string,
    opts: {
        method: "POST" | "GET";
        body: any;
        fetch: (
            input: RequestInfo | URL,
            init?: RequestInit
        ) => Promise<Response>;
    } = {
        method: "POST",
        body: {},
        fetch: fetch,
    }
): Promise<T> => {
    // console.log("fetching", url, opts, "\n")
    let input = browser ? url : `http://127.0.0.1:8080${url}`;
    const req = await opts.fetch(input, {
        method: opts.method,
        headers: {
            "Content-Type": "application/json",
        },
        body: opts.method == "POST" ? JSON.stringify(opts.body) : undefined,
    });
    if (!req.ok) {
        console.log(req);
        throw new Error(
            `failed to fetch ${url} with ${JSON.stringify(opts.body)}`
        );
    }

    return req.json();
};
