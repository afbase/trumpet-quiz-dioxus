# Trumpet Fingering Quiz

Flash cards for B♭ trumpet fingerings, written with [Dioxus](https://dioxuslabs.com/) and compiled to WebAssembly.

A note appears on a treble staff for three seconds. Its name and fingering then appear underneath for three seconds. Then a new note is picked at random, and so on forever.

The notes and fingerings are those of `trumpet-fingering-chart.webp`: written F♯3 up to C6. Notes with an accidental are shown as a sharp or as a flat at random.

## Layout

```
src/
├─ main.rs     # the app: timing loop, staff drawing, answer panel
├─ notes.rs    # note spelling, staff positions and the fingering table (with tests)
├─ glyphs.rs   # clef, notehead and accidental outlines
├─ style.css   # styles, inlined into the app at compile time
index.html     # page used by `dx serve` and `dx build`
```

## Running

```bash
dx serve          # dev server with hot reload
cargo test        # check the fingering table and staff positions
dx build --release --platform web
```

The release build lands in `target/dx/trumpet-quiz-dioxus/release/web/public/`.

## Embedding

The app mounts into the element with id `trumpet-quiz-root` and brings its own styles, so a host page needs only that element and the script:

```html
<div id="trumpet-quiz-root"></div>
<script type="module">
    import './trumpet-quiz-dioxus-dxh….js';
</script>
```

The generated JS loads its `.wasm` from an absolute `/assets/` path, which the host has to rewrite if it serves the files from anywhere else.

On every push to `main`, `.github/workflows/build.yml` publishes the build as the image `ghcr.io/afbase/trumpet-quiz-dioxus:latest` (files under `/dist/trumpet-quiz-dioxus/`) and sends a `trumpet-quiz-updated` dispatch to [afbase.github.io](https://github.com/afbase/afbase.github.io), whose deploy workflow copies the files into a blog post and does that rewrite. The dispatch needs a `CROSS_REPO_TOKEN` secret in this repository.

## Credits

The notation glyphs are outlines from [Bravura](https://github.com/steinbergmedia/bravura), © Steinberg Media Technologies GmbH, used under the SIL Open Font License 1.1 (`LICENSE-Bravura.txt`).
