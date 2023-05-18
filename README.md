# amplitude

This is a thingy for learning programming I sure do hope i finish this.

You can find the articles in the [amplitude_articles] repository.

 - `amplitude` contains the main program / web server
 - `amplitude_markdown` contains the markdown parser.
 - `amplitude_common` contains the common code shared between the two.

## Building

```bash
cargo r            # to clone the articles from `amplitude_articles`
cargo r -- --local # if you want to use your local files

cd web
npm i
npm run dev # the rust server AND the sveltekit server must be both running
```
[amplitude_articles]: https://github.com/rcsc/amplitude_articles.git