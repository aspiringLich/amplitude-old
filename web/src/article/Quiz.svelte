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
    let flyOut = { y: "15em", duration: 1000, easing: quadInOut };
    
    function flyIn(node, { duration }) {
        return {
            duration,
            css: t => {
                t = quadInOut(t);
                const y = (1 - t) * -15 - 7.5;
                return `
                transform: translateY(${y}em);
                opacity: ${t}
                `;
            }
        }
    }

    let questions = fetchQuiz().then((q) => q.questions);
    let n = -1;
</script>

{#await questions}
    <h3>Loading Quiz...</h3>
{:then questions}
    <div id="quiz" class={n >= 0 && n < questions.length ? "active" : ""}>
        {#if n == -1}
            <div style="text-align: center" out:fly={flyOut}>
                <h2>Quiz</h2>
                <h4>
                    {`${questions.length} question${
                        questions.length == 1 ? "" : "s"
                    }`}
                </h4>
                <Button hue={120} sat={50} onclick={() => n++}>Gaming</Button>
            </div>
        {:else if n < questions.length}
            <div id="container" in:flyIn="{{duration: 1000}}">
                <div id="left">
                    {@html questions[n].question}
                </div>
                <div id="right">
                    
                </div>
            </div>
        {:else}
            <!-- else content here -->
        {/if}
    </div>
{/await}

<style lang="scss">
    #quiz {
        transition: 0.5s;
        border-radius: 4px;
        border: 1px solid hsl(0, 0, 90%);
        padding: 16px 8px;
        height: 7.5em;
        overflow: hidden;

        &.active {
            height: 15em;
        }
    }
    
    #container {
        height: 100%;
    }
    
    #left {
        // resize: horizontal;
        width: 50%;
        overflow-y: scroll;
        border-right: 1px solid hsl(0, 0, 90%);
        height: 100%;
    }
    
    #right {
        width: 100%;
        height: 100%;
    }
</style>
