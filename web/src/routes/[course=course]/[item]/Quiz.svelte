<script lang="ts">
    import { fetchApi } from "$lib/fetch";
    import { renderArticle } from "./item";
    import { afterUpdate, onMount } from "svelte";
    import { ChevronLeft, ChevronRight } from "radix-icons-svelte";
    import Admonition from "./Admonition.svelte";

    // Props
    export let id: string;

    // Types
    type Questions = {
        question: string;
        answers: {
            text: string;
            response: string;
            correct: boolean;
        }[];
    }[];
    type Answers = { correct: boolean; num: number }[];

    // Intersection Observer
    let observer = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                fetchApi("/api/item", {
                    method: "POST",
                    body: { id: `${window.location.pathname}/${id}` },
                }).then((res) => {
                    questions = (res as any).questions;
                    answers = Array(questions.length);
                    n = 0;
                });
                observer.disconnect();
                observer = undefined;
            }
        });
    });

    // Local
    let questions: Questions;
    let answers: Answers;

    let container: HTMLElement;
    let n = -1;
    let prev_n = n;

    let selected: number;

    // initialization
    const try_init = () => observer && observer.observe(container);
    onMount(try_init);

    afterUpdate(() => {
        // only run when n is changed
        if (prev_n == n) return;
        prev_n = n;

        renderArticle(container);
    });

    // Button funcs
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
</script>

<svelte:window on:scroll={try_init} />

<blockquote class="container" bind:this={container}>
    <div class="buttons">
        <button on:click={dec} disabled={n <= 0}>
            <ChevronLeft />
        </button>
        <button
            on:click={submit}
            disabled={selected === undefined || answers[n] !== undefined}
        >
            Submit
        </button>
        <button
            on:click={inc}
            disabled={!questions || n >= questions.length - 1}
        >
            <ChevronRight />
        </button>
    </div>
    {#if n == -1}
        <h1>Loading...</h1>
    {:else}
        {@const answered = answers[n] !== undefined}
        <div class="question">
            {@html questions[n].question}

            {#each questions[n].answers as answer, i}
                <blockquote
                    class="choice flex pl-3"
                    class:selected={i == selected}
                    class:correct={answered && answer.correct}
                    class:incorrect={answered && !answer.correct}
                >
                    <input
                        type="radio"
                        class="mr-3"
                        value={i}
                        id={i.toString()}
                        name={id}
                        disabled={answered}
                        bind:group={selected}
                        on:click={gen_deselect(i)}
                    />
                    <label class="w-max" for={i.toString()}>
                        {@html answer.text}
                    </label>
                </blockquote>
            {/each}

            {#if answered}
                {@const answer = questions[n].answers[answers[n].num]}
                <Admonition type={answer.correct ? "correct" : "incorrect"}>
                    {@html answer.response}
                </Admonition>
            {/if}
        </div>
    {/if}
</blockquote>
