<script lang="ts">
    import Button from "$lib/components/Button.svelte";
    import { fetchApi } from "$lib/utils";
    import { getArticle, renderArticle } from "./article";
    import { afterUpdate, onMount } from "svelte";
    import { ChevronLeft, ChevronRight } from "radix-icons-svelte";

    export let id: string;

    let observer = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                init();
                observer.disconnect();
                observer = undefined;
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
    let answers: { correct: boolean; num: number }[];

    let container: HTMLElement;
    let n = -1;
    let prev_n = n;

    function try_init() {
        if (observer) observer.observe(container);
    }

    onMount(try_init);

    async function init() {
        fetchApi(`/api/quiz`, {
            method: "POST",
            body: { id, article: getArticle() },
        }).then((res) => {
            n = 0;
            questions = (res as any).questions;
            questions.push(questions[0]);
            answers = Array(questions.length);
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

    const submit = () => {
        if (selected === undefined) return;
        answers[n] = {
            correct: questions[n].answers[selected].correct,
            num: selected,
        };
    };

    const inc = () => (selected = answers[++n]?.num);

    const dec = () => (selected = answers[--n]?.num);
    let selected: number;
</script>

<svelte:window on:scroll={try_init} />

<blockquote class="container" bind:this={container}>
    {#if n == -1}
        <h1>Loading...</h1>
    {:else}
        {@const answered = answers[n] !== undefined}
        <div class="buttons">
            <Button onclick={dec} enabled={n > 0}>
                <ChevronLeft />
            </Button>
            <Button
                onclick={submit}
                enabled={selected !== undefined && answers[n] === undefined}
            >
                Submit
            </Button>
            <Button onclick={inc} enabled={n < questions.length - 1}>
                <ChevronRight />
            </Button>
        </div>
        <div class="question">
            {@html questions[n].question}
            
            {#if answered}
                <blockquote class="response">
                    {@html questions[n].answers[answers[n].num].response}
                </blockquote>
            {/if}

            {#each questions[n].answers as answer, i}
                <blockquote
                    class="choice"
                    class:selected={i == selected}
                    class:correct={answered && answer.correct}
                    class:incorrect={answered && !answer.correct}
                >
                    <input
                        type="radio"
                        value={i}
                        id={i.toString()}
                        name={id}
                        disabled={answered}
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

<style lang="scss">
    $c-padding: 0.5em;
    $i-size: 1em;

    @mixin border($color) {
        border: calc($i-size / 2) solid var($color);
    }

    @mixin input($unchecked, $checked) {
        input {
            @include border($unchecked);
        }

        .response {
            background-color: $unchecked;
        }

        &.selected input {
            @include border($checked);
        }
    }

    .response {
        margin: 1em 0;
    }

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
            input:checked {
                @include border(--blue-medium);
            }
        }

        &.incorrect {
            background-color: var(--red-light__);
            @include input(--red-light__, --red-medium);
        }

        &.correct {
            background-color: var(--green-light__);
            @include input(--green-light__, --green-medium);
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
