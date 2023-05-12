
export function urlQuery(): { [key: string]: string } {
    let query = window.location.href.split("?", 2)[1];
    if (!query) return {};
    query = query.split("#", 2)[0];
    let out: any = {};

    for (let param of query.split(";")) {
        let [key, value] = param.split("=");
        out[key] = value;
    }
    return out;
}

export function urlParts(): string[] {
    return window.location.href
        .split("?", 2)[0]
        .split("#", 2)[0]
        .split("/")
        .slice(2);
}
