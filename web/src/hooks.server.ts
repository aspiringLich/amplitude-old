import type { Handle, HandleFetch } from "@sveltejs/kit";
import { browser } from "$app/environment";

// export const handleFetch = (async ({ request, fetch }) => {
//     if (!browser) {
//         if (request.url.startsWith("http://sveltekit-prerender/api")) {
//             // clone the original request, but change the URL
//             request = new Request(
//                 new URL(request.url.replace(
//                     "http://sveltekit-prerender/",
//                     "http://127.0.0.1:8080/"
//                 )),
//                 request
//             );
//         }
//     }
//     // console.log(request);
//     return fetch(request);
// }) satisfies HandleFetch;
