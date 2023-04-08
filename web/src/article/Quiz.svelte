<script lang="ts">
    import Button from "../widgets/Button.svelte";
    import { afterUpdate } from "svelte";
    import { renderComponents } from "./article";

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
        if (!a.ok) {
            throw new Error("failed to fetch quiz");
        }

        return a.json();
    }
    let flex_direction = "row";
    let questions = fetchQuiz().then((q) => q.questions);
    let n = -1;
    let container_element: HTMLElement;

    let prev_n = n;
    afterUpdate(() => {
        if (container_element == undefined) return;
        if (prev_n == n) return;
        prev_n = n;
        renderComponents(container_element);
    });

    onresize = () => {
        if (window.innerWidth < 800) {
            flex_direction = "column";
        } else {
            flex_direction = "row";
        }
    };

    let selected = undefined;
    $: submit_enabled = selected != undefined;

    let answers = {};
</script>

{#await questions}
    <div
        id="quiz"
        style:--flex-direction={flex_direction}
        style:--max-width={flex_direction == "row" ? "50%" : "100%"}
    >
        <div id="start">
            <Button
                hue={120}
                sat={50}
                onclick={() => {
                    n++;
                }}
                disabled={true}
            >
                Start Quiz
            </Button>
            <h2>Quiz</h2>
            <h4>Loading...</h4>
        </div>
    </div>
{:then questions}
    <div
        id="quiz"
        style:--flex-direction={flex_direction}
        style:--max-width={flex_direction == "row" ? "50%" : "100%"}
    >
        {#if n == -1}
            <div id="start">
                <Button
                    hue={120}
                    sat={50}
                    onclick={() => {
                        n++;
                    }}
                >
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
                <div>
                    <Button
                        hue={120}
                        sat={50}
                        disabled={n == 0}
                        onclick={() => {
                            n--;
                            selected = answers[n];
                        }}
                        >&lt; Back
                    </Button>
                </div>
                <div>
                    <Button
                        hue={120}
                        sat={50}
                        onclick={() => {
                            n = -1;
                            prev_n = -1;
                            selected = undefined;
                            answers = {};
                        }}
                        >Reset
                    </Button>
                </div>
                <div>
                    {#if answers[n] == undefined}
                        <Button
                            hue={120}
                            sat={50}
                            disabled={!submit_enabled}
                            onclick={() => (answers[n] = selected)}
                            >Submit
                        </Button>
                    {:else}
                        <Button
                            hue={120}
                            sat={50}
                            disabled={n + 1 == questions.length}
                            onclick={() => {
                                n++;
                                selected = answers[n];
                            }}
                            >Next &gt;
                        </Button>
                    {/if}
                </div>
            </div>
            <h3 style:margin-left="16px">Question {n + 1}</h3>
            <div id="container" bind:this={container_element}>
                <div id="left">
                    {@html questions[n].question}
                </div>
                <div id="right">
                    {#each questions[n].answers as answer, i}
                        {@const exists = answers[n] != undefined}
                        <div
                            id="box"
                            class={!exists
                                ? ""
                                : answer.correct === true
                                ? "correct"
                                : "incorrect"}
                        >
                            <div
                                id="input"
                                on:keypress
                                on:click={() => {
                                    if (!exists)
                                        selected =
                                            selected != i ? i : undefined;
                                }}
                            >
                                <input
                                    type="radio"
                                    bind:group={selected}
                                    name={n.toString()}
                                    value={i}
                                    disabled={exists}
                                />
                                <label for={n.toString()}>
                                    {@html answer.text}
                                </label>
                            </div>
                            {#if exists}
                                <div style:margin="0.5em 0 0 2em">
                                    {(answer.correct === true
                                        ? "✔ Correct"
                                        : "✘ Incorrect") +
                                        (answer.response == "" ? "!" : ":")}
                                    {@html answer.response}
                                </div>
                            {/if}
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
        justify-content: stretch;
        padding: 8px 16px;

        div {
            display: flex;
            flex: 1;
            &:nth-child(1) {
                justify-content: start;
            }

            &:nth-child(2) {
                justify-content: center;
            }

            &:nth-child(3) {
                justify-content: end;
            }
        }
    }

    #box {
        border-radius: 4px;
        border: 1px solid hsl(0, 0%, 90%);
        background: hsl(0, 0%, 97%);
        margin: 8px 0;
        padding: 8px;
        align-items: center;
        user-select: none;

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
    }

    #input {
        // height: 25%;
        display: flex;
        flex-direction: row;
        // transition: 0.2s ease-in-out;

        :nth-child(2) {
            margin-left: 0.5em;
            width: 100%;
        }
    }

    h3 {
        margin: 0;
    }

    #container {
        height: calc(100% - 2em);
        display: flex;
        // column-count: 2;
        flex-direction: var(--flex-direction);
        flex-grow: 0;
        flex-shrink: 0;
        flex-basis: 50%;
    }

    #left {
        height: 100%;
        padding: 0px 16px;
        flex: 1;
        box-sizing: border-box;
        max-width: var(--max-width);
    }

    #right {
        flex-direction: column;
        padding: 0px 16px 8px 16px;
        flex: 1;
    }
</style>
