<script lang="ts">
    export let id: string;
    export let course: string;
    export let article: string;

    import Button from "../widgets/Button.svelte";
    import Icon from "../widgets/Icon.svelte";
    import { renderComponents } from "./article";

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

    let questions = fetchQuiz().then((q) => q.questions);
    let container_element: HTMLElement;
    let quiz_element: HTMLElement;

    // render the quiz markdown whenever you change the question
    $: if (container_element != undefined && n >= 0) {
        renderComponents(container_element);
    }

    // width formatting stuffs
    let width: number;
    $: layout = width > 600 ? "horizontal" : "vertical";

    function button(onclick, enabled = true) {
        return {
            color: "green",
            onclick,
            enabled,
        };
    }

    let dec = () => {
        n--;
    };
    let reset = () => {
        n = -1;
        answers = {};
    };
    let inc = () => {
        n++;
    };
    let submit = () => {
        answers[n] = selected;
    };

    let n = -1;
    let answers = {};
    $: selected = answers[n];
</script>

<svelte:window on:resize={() => (width = quiz_element.clientWidth)} />

{#await questions}
    <div id="quiz" bind:this={quiz_element} bind:clientWidth={width}>
        <div id="start">
            <Button {...button(inc, false)}>Start Quiz</Button>
            <h2>Quiz</h2>
            <h4>Loading...</h4>
        </div>
    </div>
{:then questions}
    {@const len = questions.length}
    <div id="quiz" bind:this={quiz_element}>
        {#if n == -1}
            <div id="start">
                <Button {...button(inc)}>Start Quiz</Button>
                <h2>Quiz</h2>
                <h4>
                    {`${len} question${len == 1 ? "" : "s"}`}
                </h4>
            </div>
        {:else if n < len}
            <div id="buttons">
                <div>
                    <Button {...button(dec, n > 0)}>
                        <Icon icon="arrow_back">Back</Icon>
                    </Button>
                </div>
                <div>
                    <Button {...button(reset)}>Reset</Button>
                </div>
                <div>
                    {#if answers[n] == undefined}
                        {@const submit_enabled = selected != undefined}
                        <Button {...button(submit, submit_enabled)}>
                            Submit
                        </Button>
                    {:else}
                        <Button {...button(inc, n + 1 == questions.length)}>
                            Next &gt;
                        </Button>
                    {/if}
                </div>
            </div>
            <h3 style:margin-left="16px">Question {n + 1}</h3>
            <div
                id="container"
                data-layout={layout}
                bind:this={container_element}
            >
                <div id="left">
                    {@html questions[n].question}
                </div>
                <div id="right">
                    {#each questions[n].answers as answer, i}
                        {@const exists = answers[n] != undefined}
                        {@const correct = exists ? answer.correct : null}
                        <div id="box" data-correct={correct}>
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
        flex-grow: 0;
        flex-shrink: 0;
        flex-basis: 50%;

        &[data-layout="vertical"] {
            flex-direction: column;

            #left {
                max-width: 100%;
            }
        }

        &[data-layout="vertical"] {
            flex-direction: row;

            #left {
                max-width: 50%;
            }
        }
    }

    #left {
        height: 100%;
        padding: 0px 16px;
        flex: 1;
        box-sizing: border-box;
    }

    #right {
        flex-direction: column;
        padding: 0px 16px 8px 16px;
        flex: 1;
    }
</style>
