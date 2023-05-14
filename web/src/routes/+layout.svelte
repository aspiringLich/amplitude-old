<script lang="ts">
    import { SvelteUIProvider } from '@svelteuidev/core';
</script>

<SvelteUIProvider>
    <slot />
</SvelteUIProvider>

<style lang="scss" global>
    @use "@styles/global";
</style>
