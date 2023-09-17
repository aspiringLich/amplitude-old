/**
 * The hierarchy between files in a Git repository.
 */
export interface GitTree {
    sha: string;
    url: string;
    truncated?: boolean;
    path?: string;
    mode?: string;
    type?: string;
    /**
     * Objects specifying a tree structure
     */
    tree?: {
        path?: string;
        mode?: string;
        type?: string;
        sha?: string;
        size?: number;
        url?: string;
        [k: string]: unknown;
    }[];
    [k: string]: unknown;
}

/**
 * The possible file modes in a Git repository.
 */
enum GitFileMode {
    Blob = "100644",
    Executable = "100755",
    Tree = "040000",
    Commit = "160000",
    Symlink = "120000",
}

/**
 * A high-level representation of an object in a Git repository.
 */
export class GitObject {
    path: string;
    mode: GitFileMode;
    type: "blob" | "tree";
    sha: string;
    url: string;
    size?: number;
    tree?: {
        [key: string]: GitObject;
    };

    getPath(path: string): GitObject | undefined {
        let components = path.split("/");
        let final = components.pop();
        let tree: GitObject = this;
        for (const component of components) {
            tree = tree.tree![component];
        }
        return tree.tree![final!];
    }

    /**
     * Create a GitObject from a GitTree.
     * @param object The GitTree to create a GitObject from.
     */
    constructor(object: GitTree) {
        this.path = object.path ?? "";
        this.mode = (object.mode as GitFileMode) ?? GitFileMode.Tree;
        this.type = (object.type as "blob" | "tree") ?? "tree";
        this.sha = object.sha;
        this.url = object.url;
        this.tree = {};

        if (object.tree) {
            let last_tree: GitObject = this;
            for (const entry of object.tree) {
                let obj = new GitObject({
                    path: entry.path,
                    mode: GitFileMode[entry.mode],
                    type: entry.type as "blob" | "tree",
                    sha: entry.sha,
                    url: entry.url,
                });

                // place the tree correctly in the filestructure
                last_tree = this;
                let components = entry.path.split("/");
                let final = components.pop();
                for (const component of components) {
                    last_tree = last_tree.tree![component];
                }
                last_tree.tree![final] = obj;

                switch (entry.type) {
                    case "tree":
                        obj.tree = {};
                        break;
                    case "blob":
                        obj.size = entry.size;
                        break;
                }
            }
        }
    }
}
