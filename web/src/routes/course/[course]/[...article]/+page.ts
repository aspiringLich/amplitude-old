import type { EntryGenerator } from "./$types";

class ArticleResponse {
    body: string;
    name: string;
}

export const load = async ({ fetch, params }): Promise<ArticleResponse> => {
    const req = await fetch("/api/article", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ article: params.article }),
    });
    if (!req.ok) {
        throw new Error("failed to fetch article");
    }

    return req.json();
};

export const entries = async (): Promise<EntryGenerator> => {
    const req = await fetch("/api/course-lists", {
        method: "POST",
    });
    if (!req.ok) {
        throw new Error("failed to fetch course lists");
    }
    
    let list: EntryGenerator = [] as any;
    
    console.log(await req);
    
    return list;
};
