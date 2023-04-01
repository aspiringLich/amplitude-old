<script>
    export let id;
    export let course;
    export let article;
    
    import { onMount } from "svelte";
    
    async function fetchQuiz() {
        const a = await fetch(`/api/quiz`, {
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
            throw new Error("Failed to fetch quiz");
        }
        return await a.json();
    }
    
    console.log(fetchQuiz());
</script>

edxdlhalsdhf