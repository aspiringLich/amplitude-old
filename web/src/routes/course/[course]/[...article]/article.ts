import type { ComponentType } from "svelte";

import {
    destroy_component,
    detach,
    insert,
    is_function,
    mount_component,
    noop,
    SvelteComponent,
} from "svelte/internal";

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
        let slots = target.childNodes.length > 0;

        if (slots) {
            props["$$slots"] = createSlots({
                default: [...target.children, { $$scope: {} }],
            });
            props["$$scope"] = {};
        }

        try {
            let { html } = (type as any).render({
                target: target.parentElement as Element,
                anchor: target,
                props,
            });
            console.log(html);
        } catch (e) {
            console.error(e);
        }

        target.remove();
    });
}