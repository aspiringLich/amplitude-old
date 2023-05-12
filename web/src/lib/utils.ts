import { browser } from "$app/environment";

export const fetchApi = async <T>(
    url: string,
    opts: {
        body: any;
        fetch: (
            input: RequestInfo | URL,
            init?: RequestInit
        ) => Promise<Response>;
    } = {
        body: {},
        fetch: fetch,
    }
): Promise<T> => {
    console.log("fetching", url, opts, "\n")
    const req = await opts.fetch(browser ? "" : "http://127.0.0.1:8080" + url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(opts.body),
    });
    if (!req.ok) {
        console.log(req);
        throw new Error(`failed to fetch ${url} with ${JSON.stringify(opts)}`);
    }

    return req.json();
};
