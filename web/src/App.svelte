<script lang="ts">
    // @ts-nocheck
    import { Route } from "tinro";
    import NotFound from "./NotFound.svelte";
    import Home from "./pages/Home.svelte";
    import Article from "./article/Article.svelte";
</script>

<head>
    <link
        rel="stylesheet"
        href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/agate.min.css"
    />
    <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/css2?family=Material+Symbols+Sharp:opsz,wght,FILL,GRAD@40,600,0,200"
    />
</head>

<main>
    <Route path="/"><Home /></Route>
    <Route path="/article/:course/:article" let:meta>
        <Article course={meta.params.course} article={meta.params.article} />
    </Route>
    <!-- <Route path="/about"><About /></Route>
  <Route path="/p/:id" let:meta><Problem id={meta.params.username} pid={meta.params.id} /></Route>
  <Route path="/g/:id" let:meta><Group id={meta.params.username} /></Route> -->
    <Route fallback><NotFound /></Route>
</main>

<style global lang="scss">
    @import "styles.scss";
</style>
