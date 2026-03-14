# Rich Text Editor — Slint + Rust + skparagraph

WYSIWYG rich text editor med Markdown som backend og skia/skparagraph til text layout.
Medieindlejring via skparagraph Placeholders. UI via Slint med custom Skia render notifier.

## Stack

- **UI**: Slint (render notifier → Skia canvas)
- **Text layout**: skia/skparagraph (`skia-safe` crate, feature `textlayout`)
- **Markdown parse**: `pulldown-cmark`
- **Markdown serialize**: custom serializer (AST → .md)
- **Async**: `tokio`
- **Serialisering**: `serde` + `serde_yaml` (YAML frontmatter)
- **Syntaks-highlight**: `syntect`
- **Billede-decode**: `image` crate

## Projektstruktur

```
src/
  main.rs               # Slint app entry, render notifier setup
  document/
    mod.rs              # Block + Inline AST typer
    parser.rs           # pulldown-cmark → AST
    serializer.rs       # AST → Markdown streng
    frontmatter.rs      # YAML frontmatter parse/serialize
  editor/
    mod.rs              # EditorState: cursor, selection, undo stack
    commands.rs         # Command trait + konkrete commands (Insert, Delete, Format…)
    undo.rs             # UndoStack med Command pattern
  renderer/
    mod.rs              # SkiaRenderer: paragraph cache, paint loop
    paragraph.rs        # Block → skparagraph ParagraphBuilder mapping
    cursor.rs           # Cursor + selection rect via getRectsForRange
    media.rs            # Placeholder layout + media cache
  media/
    mod.rs              # MediaManager: lazy load, thumbnail cache
ui/
  editor.slint          # EditorCanvas komponent, toolbar, statusbar
assets/                 # Lokale media assets
docs/
  architecture.md       # Se @docs/architecture.md
  skparagraph-api.md    # Se @docs/skparagraph-api.md
```

## Build-kommandoer

```bash
# Byg (debug)
cargo build

# Byg (release)
cargo build --release

# Kør editor
cargo run

# Test
cargo test

# Test enkelt modul
cargo test document::

# Clippy (kør ALTID før commit)
cargo clippy -- -D warnings

# Format (kør ALTID før commit)
cargo fmt
```

## Skia / skparagraph build-krav

skparagraph kræver at Skia bygges med specifikke flags. Verificer at `skia-safe` er kompileret med:

```toml
[dependencies]
skia-safe = { version = "0.75", features = ["textlayout", "svg"] }
```

Skia skal have `skia_use_icu=true` for korrekt Unicode, line-breaking og word boundaries.
Hvis build fejler på `skparagraph`, tjek at ICU er til stede på systemet:
- **Linux**: `apt install libicu-dev`
- **macOS**: `brew install icu4c`

## Kernekoncepter

### Dokumentmodel (Block/Inline AST)

Al redigering sker på AST'en — aldrig direkte på Markdown-strengen.
Markdown parses til AST ved åbning, serialiseres til Markdown ved gem.

```
Document
  └── Vec<Block>
        ├── Paragraph(Vec<Inline>)
        ├── Heading { level: u8, content: Vec<Inline> }
        ├── CodeBlock { lang, code }
        ├── Image { alt, path, caption }
        ├── Video { path, poster }
        └── BulletList(Vec<Vec<Inline>>)

Inline
  ├── Text(String)
  ├── Bold(Vec<Inline>)
  ├── Italic(Vec<Inline>)
  ├── Code(String)
  └── Link { text, url }
```

### skparagraph mapping

Hvert `Block` mappes til én `skia::textlayout::Paragraph`.
`Image`/`Video` blokke bruger `addPlaceholder()` med billedets dimensioner.
Media tegnes oven på placeholder-rektanglet via `getRectsForPlaceholders()`.

### Cursor og selektion

- Cursor-position: `getGlyphPositionAtCoordinate(mouse_x, mouse_y)` → `PositionWithAffinity`
- Cursor-rect: `getClosestGlyphClusterAt(dx, dy, &mut glyph_info)` → `glyph_info.fBounds`
- Selektion highlight: `getRectsForRange(start, end, RectHeightStyle::Tight, RectWidthStyle::Tight)`
- Word boundary (dobbelt-klik): `getWordBoundary(offset)`

### Undo/redo

Command pattern. `UndoStack::apply(cmd, doc)` executor og pusher til history.
`undo()` og `redo()` traverserer history med cursor-index.
Alle redigeringsoperationer SKAL gå igennem `UndoStack::apply()`.

### Slint render notifier

Slint eksponerer en `rendering_notifier` callback med adgang til Skia canvas.
Al skparagraph-rendering sker her — ikke i Slint-komponenter direkte.
Slint-komponenter håndterer kun: toolbar-klik, tastatur-events, vinduesstørrelse.

## Kode-konventioner

- Rust edition 2021
- `cargo fmt` og `cargo clippy -D warnings` skal passere rent
- Ingen `unwrap()` i produktionskode — brug `?` eller eksplicit fejlhåndtering
- `thiserror` til domæne-fejltyper, `anyhow` til applikationsniveau
- Alle offentlige typer skal have doc-kommentarer (`///`)
- Test placeres i samme fil som koden (`#[cfg(test)]` modul nederst)
- Integrationstests i `tests/`

## Vigtige advarsler

- **Byg IKKE direkte på Markdown-strengen** — al mutation går via AST + Command pattern
- **`layout()` er dyrt** — kald kun `layout(width)` ved faktisk bredde-ændring eller tekstmutation, ikke ved ren repaint
- **`updateForegroundPaint()`** kan opdatere tekstfarve uden `layout()`-kald — brug dette til hover/selektion
- **Placeholder-index** skal holdes synkroniseret med `MediaManager`'s asset-liste — de er positionsbaserede
- **ICU-afhængighed**: `getWordBoundary()` og korrekt line-breaking kræver ICU til stede ved build-tid
- **Slint render notifier** må ikke blokere — tung I/O (billedload) skal ske asynkront via `tokio`

## Detaljeret dokumentation

- Arkitektur og dataflow: @docs/architecture.md
- Komplet skparagraph API-reference: @docs/skparagraph-api.md
