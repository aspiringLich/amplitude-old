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
    let n = -1;
</script>

{#await questions}
    <h3>Loading Quiz...</h3>
{:then questions}
    <Box id="quiz">
        {#if n == -1}
            <div style="text-align: center">
                <h2>Quiz</h2>
            </div>
        {:else if n < questions.length}
            <!-- else if content here -->
        {:else}
            <!-- else content here -->
        {/if}
    </Box>
{/await}

<style lang="scss">
</style>
