<script lang="ts">
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

    let questions = fetchQuiz();
</script>

<div id="quiz" class="box">
    {#await questions}
        <h3>Loading Quiz...</h3>
    {:then questions}
        {#each questions.questions as question}
            {@html question.question}
            <ul>
                {#each question.answers as answer}
                    <li>{@html answer.text}</li>
                {/each}
            </ul>
        {/each}
    {/await}
</div>

<style lang="scss">
    #quiz {
        :global(p) {
            margin-top: 4px;
            margin-bottom: 4px;
        }
    }
</style>
