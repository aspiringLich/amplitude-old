<script>
  // @ts-nocheck
  import { Route } from "tinro";
  import NotFound from "./NotFound.svelte";
  import Home from "./pages/Home.svelte";
  import Article from "./pages/article/Article.svelte";
</script>

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

<style lang="scss">
  @font-face {
    font-family: "Noto Sans";
    src: local("assets/fonts/NotoSans-Regular.ttf");
  }
  
  @font-face {
    font-family: "Fira Code";
    src: local("assets/fonts/FiraCode-Regular.ttf");
  }
  
    :global(*) {
      font-family: "Noto Sans" "Fira Code"
    }
    
    p {
      font-family: "Noto Sans";
    }
    
    h1,h2,h3,h4,h5,h6 {
      font-family: "Fira Code";
    }
  
</style>
