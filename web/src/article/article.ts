import type { ComponentType } from "svelte";

import Code from "./Code.svelte";
import Admonition from "./Admonition.svelte";

import {
    destroy_component,
    detach,
    insert,
    is_function,
    mount_component,
    noop,
    SvelteComponent,
} from 'svelte/internal';

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
        }
        else {
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
};

// for slots: currently only works for a single default slot
export function renderComponent(
    doc: HTMLElement,
    query: string,
    type: ComponentType,
    extra_props: any = {}
) {
    doc.querySelectorAll(query).forEach((target) => {
        let props = {};
        for (const attr of target.attributes) {
            props[attr.name] = attr.value;
        }
        for (const prop in extra_props) {
            props[prop] = extra_props[prop];
        }
        let slots = target.childNodes.length > 0;

        if (slots) {
            props["$$slots"] = createSlots({ default: [...target.children, { $$scope: {} }] });
            props["$$scope"] = {};
        }

        try {
            new type({
                target: target.parentElement,
                anchor: target,
                props,
            });
        } catch (e) {
            console.error(e);
        }

        target.remove();
    });
}

// renders a document from a html str
export function renderComponents(el: HTMLElement, extra_props: any = {}) {
    renderComponent(el, "pre", Code);
    renderComponent(el, "Admonition", Admonition);
}

export function smoothAnchor(anchor: Element) {
    anchor.addEventListener("click", function (e) {
        e.preventDefault();

        document.querySelector(this.getAttribute("href")).scrollIntoView({
            behavior: "smooth",
        });
    });
}
