<script lang="ts">
    import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
    import "@skeletonlabs/skeleton/styles/skeleton.css";
    import "../app.postcss";
    import "../themes/base.postcss";
    import "../themes/colors.postcss";
    
    import hljs from 'highlight.js';
    import { storeHighlightJs } from '@skeletonlabs/skeleton';
    import 'highlight.js/styles/agate.css';

    storeHighlightJs.set(hljs);
</script>

<slot />

