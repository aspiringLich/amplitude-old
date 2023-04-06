<script lang="ts">
    import Box from "../widgets/Box.svelte";

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

<Box hue={0}>
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
</Box>

<style lang="scss">
</style>
