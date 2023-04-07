<script lang="ts">
    import Button from "../widgets/Button.svelte";
    import { fly } from "svelte/transition";
    import { quadInOut } from "svelte/easing";

    export let id: string;
    export let course: string;
    export let article: string;

    async function fetchQuiz() {
        const a = await fetch("/api/quiz", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                id,
                course,
                article,
            }),
        });

        return a.json();
    }

    let questions = fetchQuiz().then((q) => q.questions);
    let n = -1;

    let selected = undefined;
    $: submit_enabled = selected != undefined;

    let answers = {};
</script>

{#await questions}
    <h3>Loading Quiz...</h3>
{:then questions}
    <div id="quiz">
        {#if n == -1}
            <div id="start">
                <Button hue={120} sat={50} onclick={() => n++}>
                    Start Quiz
                </Button>
                <h2>Quiz</h2>
                <h4>
                    {`${questions.length} question${
                        questions.length == 1 ? "" : "s"
                    }`}
                </h4>
            </div>
        {:else if n < questions.length}
            <div id="buttons">
                <Button
                    hue={120}
                    sat={50}
                    onclick={() => {
                        n--;
                        selected = answers[n];
                    }}>&lt; Back</Button
                >
                <Button
                    hue={120}
                    sat={50}
                    onclick={() => {
                        n = -1;
                        selected = undefined;
                        answers = {};
                    }}>Reset</Button
                >
                {#if answers[n] == undefined}
                    <Button
                        hue={120}
                        sat={50}
                        disabled={!submit_enabled}
                        onclick={() => (answers[n] = selected)}>Submit</Button
                    >
                {:else}
                    <Button
                        hue={120}
                        sat={50}
                        disabled={n + 1 == questions.length}
                        onclick={() => {
                            n++;
                            selected = answers[n];
                        }}>Next &gt;</Button
                    >
                {/if}
            </div>
            <div id="container">
                <div id="left">
                    <h3>Question {n + 1}</h3>
                    {@html questions[n].question}
                </div>
                <div id="right">
                    {#each questions[n].answers as answer, i}
                        <div
                            id="input"
                            on:keypress
                            on:click={() =>
                                (selected = selected != i ? i : undefined)}
                            class={answers[n] == undefined
                                ? ""
                                : answer.correct === true
                                ? "correct"
                                : "incorrect"}
                        >
                            <input
                                type="radio"
                                bind:group={selected}
                                name={n.toString()}
                                value={i}
                            />
                            <label for={n.toString()}>
                                {@html answer.text}
                                {#if answers[n] != undefined}
                                    <br />
                                    {answer.correct === true
                                        ? "✔ Correct: "
                                        : "✘ Incorrect: "}
                                    {@html answer.response}
                                {/if}
                            </label>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>
{/await}

<style lang="scss">
    #quiz {
        transition: 0.5s;
        border-radius: 4px;
        border: 1px solid hsl(0, 0%, 90%);
        overflow: hidden;

        :global(p) {
            margin: 12px 0;
        }
    }

    #start {
        text-align: center;
        padding: 8px 16px;
    }

    #buttons {
        display: flex;
        justify-content: space-between;
        padding: 8px 16px;
    }

    #input {
        // height: 25%;
        border-radius: 4px;
        border: 1px solid hsl(0, 0%, 90%);
        background: hsl(0, 0%, 97%);
        margin: 8px 0;
        padding: 8px;
        align-items: center;
        user-select: none;
        // transition: 0.2s ease-in-out;

        &.correct {
            border: 1px solid hsl(120, 50%, 90%);
            background: hsl(120, 50%, 97%);
            color: hsl(120, 50%, 50%);
        }

        &.incorrect {
            border: 1px solid hsl(0, 50%, 90%);
            background: hsl(0, 50%, 97%);
            color: hsl(0, 50%, 50%);
        }

        &:hover {
            filter: saturate(0.97) brightness(1.015);
        }

        &:active {
            filter: saturate(0.9) brightness(0.98);
        }
    }

    #container {
        height: calc(100% - 2em);
        display: flex;
        column-count: 2;
        flex-direction: row;
        flex-grow: 0;
        flex-shrink: 0;
        flex-basis: 50%;
    }

    #left {
        height: 100%;
        padding: 8px 16px;
        flex: 1;
    }

    #right {
        flex-direction: column;
        padding: 0px 16px 8px 16px;
        flex: 1;
    }
</style>
