<script lang="ts">
    import Button from "@src/lib/components/Button.svelte";
    import { fetchApi } from "@src/lib/utils";
    import { getArticle } from "@src/routes/course/[course]/[article]/article";

    export let id: string;

    let observer = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                init();
                observer.disconnect();
            }
        });
    });

    let questions: {
        question: string;
        answers: {
            answer: string;
            correct: boolean;
        }[];
    }[];

    let container: HTMLElement;
    let n = -1;
    async function init() {
        fetchApi(`/api/quiz`, {
            method: "POST",
            body: { id, article: getArticle() },
        }).then((res) => {
            questions = (res as any).questions;
            n = 0;
        });
    }
</script>

<div class="container" bind:this={container}>
    {#if n == -1}
        <h1>Loading...</h1>
    {:else}
        {@const question = questions[n].question}
        {@const answers = questions[n].answers}

        <div class="question">
            {@html question}
        </div>
    {/if}
</div>

<svelte:window on:scroll={() => observer.observe(container)} />

<style lang="scss">
    .question {
        width: 100%;
    }
    
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
    }
</style>
