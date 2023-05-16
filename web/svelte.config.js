import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';
import sveltePreprocess from 'svelte-preprocess';
import path from 'path';

function importer(url) {
	for (const [alias, aliasPath] in
		[
			["$lib", "./src/lib"],
			["@src", "./src"],
		]) {
		if (url.startsWith(alias)) {
			return {
				file: url.replace(alias, aliasPath),
			};
		}
	}
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [
		vitePreprocess({
			style: {
				css: {
					preprocessorOptions: {
						scss: {
							importer,
						}
					}
				}
			}
		}), 
		sveltePreprocess({
			includePaths: ["src/styles"],
			scss: {
				importer,
			}
		})
	],
	// experimental: {
	// 	useVitePreprocess: true,
	// },

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter(),
	},
};

export default config;
