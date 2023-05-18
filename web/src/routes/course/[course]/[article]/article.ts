import type { ComponentType } from "svelte";
import {
    compute_rest_props,
    destroy_component,
    detach,
    insert,
    is_function,
    mount_component,
    noop,
    SvelteComponent,
} from "svelte/internal";
import Quiz from "./Quiz.svelte";
import Code from "./Code.svelte";
import Admonition from "./Admonition.svelte";
// import svelte from "svelte/compiler"

// https://github.com/sveltejs/svelte/issues/2588
function createSlots(slots) {
    const svelteSlots = {};

    for (const slotName in slots) {
        svelteSlots[slotName] = [createSlotFn(slots[slotName])];
    }

    function createSlotFn([ele, props = {}]) {
        if (is_function(ele) && ele.prototype instanceof SvelteComponent) {
            let component;
            return function () {
                return {
                    c: noop,
                    m(target, anchor) {
                        component = new ele({ target, props });
                        mount_component(component, target, anchor, null);
                    },
                    d(detaching) {
                        destroy_component(component, detaching);
                    },
                    l: noop,
                };
            };
        } else {
            return function () {
                return {
                    c: noop,
                    m: function mount(target, anchor) {
                        insert(target, ele, anchor);
                    },
                    d: function destroy(detaching) {
                        if (detaching) {
                            detach(ele);
                        }
                    },
                    l: noop,
                };
            };
        }
    }
    return svelteSlots;
}

// for slots: currently only works for a single default slot
export function renderComponent(
    doc: HTMLElement,
    query: string,
    type: ComponentType
) {
    doc.querySelectorAll(query).forEach((target) => {
        let props = {};
        for (const attr of target.attributes) {
            props[attr.name] = attr.value;
        }

        if (target.childElementCount) {
            props["$$slots"] = createSlots({
                default: [...target.children, { $$scope: {} }],
            });
            props["$$scope"] = {};
        }

        try {
            new type({
                target: target.parentElement as Element,
                anchor: target,
                props,
            });
        } catch (e) {
            console.error(e);
        }

        target.remove();
    });
}

export function getArticle() {
    return window.location.pathname.split("/")[3];
}

export function renderArticle(body: HTMLElement) {
    renderComponent(body, "pre", Code);
    renderComponent(body, "admonition", Admonition);
    renderComponent(body, "quiz", Quiz);

    // turn all h2s into links to themselves
    body.childNodes.forEach((element: HTMLElement) => {
        if (element.localName != "h2") return;

        let id = element.textContent.toLowerCase().replace(/[^a-z0-9]/g, "-");
        element.id = id;
        element.innerHTML = `<a href="#${id}">${element.innerHTML}</a>`;
    });
}

// The response from the server containing information about the article
export class ArticleResponse {
    body: string;
    config: {
        title: string;
        id: string;
    };
}