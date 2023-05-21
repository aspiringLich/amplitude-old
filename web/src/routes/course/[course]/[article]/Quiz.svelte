<script lang="ts">
    import Button from "@src/lib/components/Button.svelte";
    import { fetchApi } from "@src/lib/utils";
    import { getArticle, renderArticle } from "./article";
    import { afterUpdate } from "svelte";

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
            text: string;
            response: string;
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

    afterUpdate(() => {
        // only run when n is changed
        if (prev_n == n) return;
        prev_n = n;

        renderArticle(container);
    });

    let prev_n = n;
</script>

<blockquote class="container" bind:this={container}>
    {#if n == -1}
        <h1>Loading...</h1>
    {:else}
        {@const question = questions[n].question}
        {@const answers = questions[n].answers}

        <div class="buttons">
            <Button>Previous</Button>
            <Button>Submit</Button>
        </div>
        <div class="question">
            {@html question}
            {#each answers as answer, i}
                <blockquote>
                    
                </blockquote>
            {/each}
        </div>
    {/if}
</blockquote>

<svelte:window on:scroll={() => observer.observe(container)} />

<style lang="scss">
    blockquote.selected {
        background-color: var(--green-l);
    }

    .buttons {
        display: flex;
        justify-content: space-between;
        padding: 8px;
        width: 100%;
    }

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
