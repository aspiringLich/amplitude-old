<script lang="ts">
    import Button from "$src/lib/components/Button.svelte";
    import { fetchApi } from "$src/lib/utils";
    import { getArticle, renderArticle } from "./article";
    import { afterUpdate } from "svelte";
    import { ChevronLeft, ChevronRight } from "radix-icons-svelte";

    export let id: string;

    let observer = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                init();
                observer.disconnect();
            }
        });
    });

    let questions: {
        question: string;
        answers: {
            text: string;
            response: string;
            correct: boolean;
        }[];
    }[];
    type Correct = true | false | undefined;
    let correct: Correct[];

    let container: HTMLElement;
    let n = -1;

    async function init() {
        fetchApi(`/api/quiz`, {
            method: "POST",
            body: { id, article: getArticle() },
        }).then((res) => {
            n = 0;
            questions = (res as any).questions;
            correct = Array(questions.length);
        });
    }

    afterUpdate(() => {
        // only run when n is changed
        if (prev_n == n) return;
        prev_n = n;

        renderArticle(container);
    });

    const gen_deselect = (i: number) => {
        return () => {
            if (selected == i) selected = undefined;
        };
    };
    function submit() {
        console.log("dalkjfkfjak");
        correct[n] = questions[n].answers[selected].correct;
        console.log(correct);
        selected = undefined;
    }
    $: console.log(selected);
    const inc = () => n++;
    const dec = () => n--;

    let selected: number;

    // $: console.log(selected)

    let prev_n = n;
</script>

<blockquote class="container" bind:this={container}>
    {#if n == -1}
        <h1>Loading...</h1>
    {:else}
        {@const question = questions[n].question}
        {@const answers = questions[n].answers}
        {@const submitted = correct[n] !== undefined}

        <div class="buttons">
            <Button onclick={dec} enabled={n > 0}>
                <ChevronLeft />
            </Button>
            <Button onclick={submit} enabled={selected !== undefined}>
                Submit
            </Button>
            <Button onclick={inc} enabled={n < questions.length - 1}>
                <ChevronRight />
            </Button>
        </div>
        <div class="question">
            {@html question}
            {#each answers as answer, i}
                <blockquote class="choice" class:selected={i == selected}>
                    <input
                        type="radio"
                        value={i}
                        id={i.toString()}
                        name={id}
                        bind:group={selected}
                        on:click={gen_deselect(i)}
                    />
                    <label for={i.toString()}>
                        {@html answer.text}
                    </label>
                </blockquote>
            {/each}
        </div>
    {/if}
</blockquote>

<svelte:window on:scroll={() => observer.observe(container)} />

<style lang="scss">
    $c-padding: 0.5em;
    $i-size: 1em;

    .choice {
        display: flex;
        align-items: center;
        flex-direction: row;

        width: 100%;
        margin: 1em 0;
        padding-left: $c-padding;

        transition: background-color 0.2s linear;

        &.selected {
            background-color: var(--blue-light_);
        }
    }

    input {
        appearance: none;

        border-radius: 50%;
        width: $i-size;
        height: $i-size;

        transition: 0.1s all linear;
        margin-right: $c-padding;
        margin-left: 0;
        position: relative;

        border: 2px solid var(--gray-medium);

        &:checked {
            border: 0.5em solid var(--blue-medium);
        }
    }

    label {
        flex-grow: 99;
    }

    .buttons {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 8px;
        width: 100%;
    }

    .question {
        width: 100%;
    }

    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
    }
</style>
