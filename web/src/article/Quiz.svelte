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

    // render the quiz markdown whenever you change the question
    $: if (container_element != undefined && n >= 0) {
        renderComponents(container_element);
    }

    // width formatting stuffs
    let width: number = window.innerWidth;
    $: layout = width > 790 ? "horizontal" : "vertical";

    let button = (onclick, enabled = true) => {
        return {
            color: "green",
            onclick,
            enabled,
        };
    };
    let answer_color = (exists: boolean, correct: boolean) => {
        if (!exists) return "2";
        if (correct) return "green";
        return "red";
    };

    let dec = () => n--;
    let inc = () => n++;
    let submit = () => (answers[n] = selected);
    let reset = () => {
        n = -1;
        answers = {};
    };
    let select = (i) => {
        if (selected == i) selected = undefined;
        else selected = i;
    };

    let n = -1;
    let answers = {};
    $: selected = answers[n];
</script>

<svelte:window on:resize={() => (width = window.innerWidth)} />

{#await questions}
    <div id="quiz">
        <div id="start">
            <Button {...button(inc, false)}>Start Quiz</Button>
            <h2>Quiz</h2>
            <h4>Loading...</h4>
        </div>
    </div>
{:then questions}
    {@const len = questions.length}
    <div id="quiz">
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
                <div style:justify-content="left">
                    <Button {...button(dec, n > 0)}>
                        <Icon icon="arrow_back">Back</Icon>
                    </Button>
                </div>
                <div style:justify-content="center">
                    <Button {...button(reset)}>Reset</Button>
                </div>
                <div style:justify-content="right">
                    {#if answers[n] == undefined}
                        {@const submit_enabled = selected != undefined}
                        <Button {...button(submit, submit_enabled)}>
                            Submit
                        </Button>
                    {:else}
                        {@const last = n + 1 < questions.length}
                        <Button {...button(inc, last)}>
                            <Icon icon="arrow_forward" reverse={true}>
                                Next
                            </Icon>
                        </Button>
                    {/if}
                </div>
            </div>
            <h3 style:margin-left="16px">Question {n + 1} / {len}</h3>
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
                        {@const correct = exists && answer.correct}
                        {@const this_selected = exists && answers[n] == i}
                        <div
                            class="input"
                            role="radio"
                            tabindex="-1"
                            aria-checked={this_selected}
                            on:keypress
                            on:click={() => !exists && select(i)}
                        >
                            <Button
                                color={answer_color(exists, correct)}
                                stretch={true}
                                enabled={!exists}
                                grayout={false}
                            >
                                <div class="answer">
                                    <input
                                        type="radio"
                                        bind:group={selected}
                                        name={n.toString()}
                                        value={i}
                                        disabled={exists}
                                    />
                                    <label
                                        for={n.toString()}
                                        class="n-border-radius"
                                    >
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
                            </Button>
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
        }
    }
    .input {
        margin: 0.5em 0;

        .answer {
            display: flex;
            align-items: center;

            input {
                margin-left: 0;
            }

            label {
                margin-left: 0.5em;
                width: 100%;
            }
        }
    }

    h3 {
        margin: 0;

        // &:after {
        //     content: " ";
        //     display: block;
        //     transform: translateY(-4px);
        //     margin-right: 16px;
        //     border: 0.5px solid black;
        // }
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
                min-width: 100%;
            }
        }

        &[data-layout="horizontal"] {
            flex-direction: row;

            #left {
                max-width: 50%;
                min-width: 50%;
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
        align-content: stretch;
    }
</style>
