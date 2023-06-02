<script lang="ts">
    import "../app.postcss";

    // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "@skeletonlabs/skeleton/styles/skeleton.css";
    import "../themes/base.postcss";
    import "../themes/cool.postcss";
    
    import hljs from 'highlight.js';
    import { storeHighlightJs } from '@skeletonlabs/skeleton';
    import 'highlight.js/styles/agate.css';

    storeHighlightJs.set(hljs);
</script>

<slot />

