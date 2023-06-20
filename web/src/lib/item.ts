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

type Props = { [name: string]: any };

// for slots: currently only works for a single default slot
export function renderComponent(
    doc: HTMLElement,
    query: string,
    type: ComponentType,
    propsCallback: (props: Props, slots: Props) => Props = (props) => props,
    slotCallback: (slots: Props) => Props = (slots) => slots
) {
    doc.querySelectorAll(query).forEach((target) => {
        let props = {};
        for (const attr of target.attributes) {
            props[attr.name] = attr.value;
        }

        if (target.childElementCount) {
            let slots: Object = {
                default: [...target.children, { $$scope: {} }],
            };
            props = propsCallback(props, slots);
            slots = slotCallback(slots);

            props["$$slots"] = createSlots(slots);
            props["$$scope"] = {};
        } else {
            props = propsCallback(props, {});
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

import Quiz from "$cmpt/article/Quiz.svelte";
import Code from "$cmpt/article/Code.svelte";
import Admonition from "$cmpt/article/Admonition.svelte";

export function renderArticle(body: HTMLElement, data?: ArticleData) {
    renderComponent(
        body,
        "pre:not(.component)",
        Code,
        (props, slots) => {
            let language =
                slots.default[0].classList[0]?.replace("language-", "") ??
                "plaintext";

            return {
                code: slots.default[0].innerHTML,
                language,
                copy: true,
            };
        },
        (slots) => {
            return {};
        }
    );
    renderComponent(body, "admonition", Admonition);
    renderComponent(body, "quiz", Quiz, (props, slots) => {
        if (!data) return props;
        return {
            data: data.quiz_data[props.id]
        }
    });

    // turn all h2s into links to themselves
    body.querySelectorAll("h2").forEach((h2) => {
        let id = h2.textContent.toLowerCase().replace(/[^a-z0-9]/g, "-");
        h2.id = id;
        h2.innerHTML = `<a href="#${id}">${h2.innerHTML}</a>`;
    });
    body.querySelectorAll("h1,h2,h3,h4,h5,h6").forEach((h) => {
        h.classList.add(h.localName);
    });
}

// The response from the server containing information about the article
export class ArticleData {
    body: string;
    title: string;
    quiz_data?: { [key: string]: QuizData };
    type?: "article";
}

export class QuizData {
    id: string;
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
        functions: {
            [key: string]: {
                inputs: string[];
                output: string;
                hidden_cases: number;
                visible_cases: number;
                tests: {
                    inputs: Object[];
                    output: Object;
                }[];
            };
        };
    };
    lang_info: {
        [key: string]: {
            code: string;
        };
    };
    type?: "exercise";
}

export type Item = ArticleData | QuizData | ExerciseData;
