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
// import svelte from "svelte/compiler"

// https://github.com/sveltejs/svelte/issues/2588
function createSlots(slots: { [name: string]: any }) {
    const svelteSlots = {};

    for (const [slotName, slot] of Object.entries(slots)) {
        if (slot) svelteSlots[slotName] = [createSlotFn(slots[slotName])];
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
    type: ComponentType,
    manipulate: (
        props: { [name: string]: any },
        slots: { [name: string]: any },
    ) => void = () => {}
) {
    doc.querySelectorAll(query).forEach((target) => {
        let props = {};
        for (const attr of target.attributes) {
            props[attr.name] = attr.value;
        }

        if (target.childElementCount) {
            let slots = {
                default: [...target.children, { $$scope: {} }],
            };
            manipulate(props, slots);

            props["$$slots"] = createSlots(slots);
            props["$$scope"] = {};
        } else {
            manipulate(props, {})
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

export function itemID() {
    return window.location.pathname.split("/").slice(1).join("/");
}

import Quiz from "$cmpt/Quiz.svelte";
import Code from "$cmpt/Code.svelte";
import Admonition from "$cmpt/Admonition.svelte";

export function renderArticle(body: HTMLElement) {
    renderComponent(body, "pre:not(.component)", Code, (props, slots) => {
        // console.log(slots);
        let language = slots.default[0].classList[0]?.replace("language-", "") ?? "plaintext";
        
        props.code = slots.default[0].innerHTML;
        props.language = language;
        slots.default = null;
    });
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
export class ArticleData {
    body: string;
    title: string;
    type?: "article";
}

export class QuizData {
    questions: {
        question: string;
        answers: {
            text: string;
            response: string;
            correct: boolean;
        }[];
    }[];
    type?: "quiz";
}

export class ExerciseData {
    config: {
        title: string;
        instructions: string;
        functions: {[key: string]: {
            inputs: string[];
            output: string;
            hidden_cases: number;
            visible_cases: number;
            tests: {
                inputs: Object[];
                output: Object;
            }[];
        }};
    };
    lang_info: {[key: string]: {
        code: string;
    }}
    type?: "exercise";
}

export type Item = ArticleData | QuizData | ExerciseData;
