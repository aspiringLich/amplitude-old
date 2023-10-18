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
export enum GitFileMode {
    Blob = "100644",
    Executable = "100755",
    Tree = "040000",
    Commit = "160000",
    Symlink = "120000",
}

export enum ModificatonStatus {
    UNCHANGED,
    ADDED,
    MODIFIED,
    DELETED,
}

export enum Error {
    /** Expected a Blob, did not get one */
    NOT_BLOB,
    /** Expected a Tree, did not get one */
    NOT_TREE,
    /** Expected this object to have a parent */
    NO_PARENT,
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
    content?: string;
    tree: {
        [key: string]: {
            status: ModificatonStatus;
            obj: GitObject;
        };
    } | null;
    parent: GitObject | null;
    modified: boolean;

    getContent(): string {
        if (this.content) return this.content;
        if (this.type == "blob") {
        } else {
            throw Error.NOT_BLOB;
        }
    }

    /**
     *
     * @param path The path of the
     * @returns A gitobject if it fin
     */
    chdir(path: string): GitObject | Error {
        let components = path.split("/");
        let final = components.pop();
        let tree: GitObject = this;
        for (const component of components) {
            if (component === "..") {
                tree = tree.parent;
                if (!tree) return Error.NO_PARENT;
            } else if (component !== ".") {
                tree = tree.tree[component]?.obj;
                if (tree?.type !== "tree") return Error.NOT_TREE;
            }
        }
        return tree?.tree?.[final]?.obj ?? Error.NOT_TREE;
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
        this.tree = null;
        this.parent = (object.parent as GitObject) ?? null;
        this.modified = false;

        if (object.tree) {
            this.tree = {};
            let last_tree: GitObject = this;
            for (const entry of object.tree) {
                let obj = new GitObject({
                    path: entry.path,
                    mode: GitFileMode[entry.mode],
                    type: entry.type as "blob" | "tree",
                    sha: entry.sha,
                    url: entry.url,
                    parent: this,
                });

                // place the tree correctly in the filestructure
                last_tree = this;
                let components = entry.path.split("/");
                let final = components.pop();
                for (const component of components) {
                    last_tree = last_tree.tree![component].obj;
                }
                last_tree.tree![final] = {
                    status: ModificatonStatus.UNCHANGED,
                    obj: obj,
                };

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
