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

## To-Do

 - [x] Reworked article / item system
 - [x] Article
   - [x] Admonitions
   - [x] Code Blocks
   - [x] Quiz
 - [ ] Exercises ~~/ Projects~~
   - [x] Code editor
   - [x] Test case generation system
   - [ ] Other languages
 - [ ] General Stuff 
   - [ ] "Home" page
   - [ ] Login page
   - [ ] Settings
   - [ ] Course page
 - [ ] Web Course Editor
   - [ ] Edit Exercises 
   - [ ] Create Exercises
   - [ ] Edit Items
   - [ ] Create Items
   - [ ] Delete Items
   - [ ] Move Items
   - [ ] Create Courses
 - [ ] Other
   - [x] Modals
   - [x] Toast notifications
   - [ ] Robust Form Components

[amplitude_articles]: https://github.com/rcsc/amplitude_articles.git