<script lang="ts">
    import { modalStore } from "@skeletonlabs/skeleton";
    import { editorSettings as settings } from "$lib/settings";
    import * as themes from "thememirror";
    
    // https://stackoverflow.com/questions/63116039/camelcase-to-kebab-case
    const kebabize = (str) =>
        str.replace(
            /[A-Z]+(?![a-z])|[A-Z]/g,
            ($, ofs) => (ofs ? "-" : "") + $.toLowerCase()
        );
</script>

<h2>Editor Settings</h2>


