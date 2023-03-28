<script>
  // @ts-nocheck
  import { Route } from "tinro";
  import NotFound from "./NotFound.svelte";
  import Home from "./pages/Home.svelte";
</script>

<main>
  <Route path="/"><Home /></Route>
  <!-- <Route path="/about"><About /></Route>
  <Route path="/p/:id" let:meta><Problem id={meta.params.username} pid={meta.params.id} /></Route>
  <Route path="/g/:id" let:meta><Group id={meta.params.username} /></Route> -->
  <Route fallback><NotFound /></Route>
</main>
