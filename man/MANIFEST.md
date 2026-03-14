# Documentation Index

Generated: 2026-03-14T15:50:30.175266800+01:00  
Project: `project`  
Coverage: **74/75** items documented (**98%**) — ⚠ **1 undocumented**

## Files

| Source File | Items | Undocumented |
|---|---|---|
| [build.rs](man/build.md) | 0 | — |
| [src/adapter/editor_state_adp.rs](man/src/adapter/editor_state_adp.md) | 8 | ✓ |
| [src/adapter/mod.rs](man/src/adapter/mod.md) | 1 | ✓ |
| [src/core/document/frontmatter.rs](man/src/core/document/frontmatter.md) | 2 | ✓ |
| [src/core/document/mod.rs](man/src/core/document/mod.md) | 15 | ✓ |
| [src/core/document/parser.rs](man/src/core/document/parser.md) | 1 | ✓ |
| [src/core/document/serializer.rs](man/src/core/document/serializer.md) | 1 | ✓ |
| [src/core/editor/commands.rs](man/src/core/editor/commands.md) | 8 | ✓ |
| [src/core/editor/mod.rs](man/src/core/editor/mod.md) | 2 | ✓ |
| [src/core/editor/undo.rs](man/src/core/editor/undo.md) | 10 | ✓ |
| [src/core/mod.rs](man/src/core/mod.md) | 2 | ✓ |
| [src/gateway/mod.rs](man/src/gateway/mod.md) | 0 | — |
| [src/lib.rs](man/src/lib.md) | 6 | ✓ |
| [src/main.rs](man/src/main.md) | 0 | — |
| [src/pal/cursor.rs](man/src/pal/cursor.md) | 0 | — |
| [src/pal/media.rs](man/src/pal/media.md) | 0 | — |
| [src/pal/mod.rs](man/src/pal/mod.md) | 3 | ✓ |
| [src/pal/paragraph.rs](man/src/pal/paragraph.md) | 0 | — |
| [src/shared/durations.rs](man/src/shared/durations.md) | 2 | ✓ |
| [src/shared/limits.rs](man/src/shared/limits.md) | 2 | ✓ |
| [src/shared/mod.rs](man/src/shared/mod.md) | 4 | ✓ |
| [src/shared/paths.rs](man/src/shared/paths.md) | 2 | ✓ |
| [src/shared/sizes.rs](man/src/shared/sizes.md) | 3 | ✓ |
| [src/ui/gui/mod.rs](man/src/ui/gui/mod.md) | 0 | — |
| [src/ui/mcp/mod.rs](man/src/ui/mcp/mod.md) | 0 | — |
| [src/ui/mod.rs](man/src/ui/mod.md) | 2 | ✓ |
| [ui/main.slint](man/ui/main.md) | 1 | ⚠ 1 |

## All Items

| Item | Kind | Source | Line | Documented |
|---|---|---|---|---|
| `EditorState` | struct | src/adapter/editor_state_adp.rs | 11 | ✓ |
| `new` | fn | src/adapter/editor_state_adp.rs | 26 | ✓ |
| `from_markdown` | fn | src/adapter/editor_state_adp.rs | 37 | ✓ |
| `apply` | fn | src/adapter/editor_state_adp.rs | 48 | ✓ |
| `undo` | fn | src/adapter/editor_state_adp.rs | 54 | ✓ |
| `redo` | fn | src/adapter/editor_state_adp.rs | 60 | ✓ |
| `to_markdown` | fn | src/adapter/editor_state_adp.rs | 66 | ✓ |
| `selected_text` | fn | src/adapter/editor_state_adp.rs | 71 | ✓ |
| `editor_state_adp` | mod | src/adapter/mod.rs | 2 | ✓ |
| `extract` | fn | src/core/document/frontmatter.rs | 7 | ✓ |
| `format` | fn | src/core/document/frontmatter.rs | 47 | ✓ |
| `parser` | mod | src/core/document/mod.rs | 2 | ✓ |
| `serializer` | mod | src/core/document/mod.rs | 4 | ✓ |
| `frontmatter` | mod | src/core/document/mod.rs | 6 | ✓ |
| `ByteRange` | struct | src/core/document/mod.rs | 10 | ✓ |
| `Document` | struct | src/core/document/mod.rs | 19 | ✓ |
| `new` | fn | src/core/document/mod.rs | 28 | ✓ |
| `to_markdown` | fn | src/core/document/mod.rs | 36 | ✓ |
| `from_markdown` | fn | src/core/document/mod.rs | 41 | ✓ |
| `char_count` | fn | src/core/document/mod.rs | 46 | ✓ |
| `word_count` | fn | src/core/document/mod.rs | 51 | ✓ |
| `block_count` | fn | src/core/document/mod.rs | 56 | ✓ |
| `Frontmatter` | struct | src/core/document/mod.rs | 69 | ✓ |
| `Block` | enum | src/core/document/mod.rs | 76 | ✓ |
| `Inline` | enum | src/core/document/mod.rs | 138 | ✓ |
| `plain_text` | fn | src/core/document/mod.rs | 158 | ✓ |
| `parse` | fn | src/core/document/parser.rs | 8 | ✓ |
| `serialize` | fn | src/core/document/serializer.rs | 6 | ✓ |
| `Command` | trait | src/core/editor/commands.rs | 6 | ✓ |
| `CommandError` | enum | src/core/editor/commands.rs | 17 | ✓ |
| `InsertTextCommand` | struct | src/core/editor/commands.rs | 46 | ✓ |
| `DeleteRangeCommand` | struct | src/core/editor/commands.rs | 89 | ✓ |
| `new` | fn | src/core/editor/commands.rs | 100 | ✓ |
| `ReplaceRangeCommand` | struct | src/core/editor/commands.rs | 145 | ✓ |
| `new` | fn | src/core/editor/commands.rs | 158 | ✓ |
| `InsertBlockCommand` | struct | src/core/editor/commands.rs | 213 | ✓ |
| `commands` | mod | src/core/editor/mod.rs | 2 | ✓ |
| `undo` | mod | src/core/editor/mod.rs | 4 | ✓ |
| `UndoStack` | struct | src/core/editor/undo.rs | 9 | ✓ |
| `new` | fn | src/core/editor/undo.rs | 18 | ✓ |
| `apply` | fn | src/core/editor/undo.rs | 27 | ✓ |
| `undo` | fn | src/core/editor/undo.rs | 50 | ✓ |
| `redo` | fn | src/core/editor/undo.rs | 60 | ✓ |
| `can_undo` | fn | src/core/editor/undo.rs | 70 | ✓ |
| `can_redo` | fn | src/core/editor/undo.rs | 75 | ✓ |
| `undo_description` | fn | src/core/editor/undo.rs | 80 | ✓ |
| `depth` | fn | src/core/editor/undo.rs | 89 | ✓ |
| `clear` | fn | src/core/editor/undo.rs | 94 | ✓ |
| `document` | mod | src/core/mod.rs | 2 | ✓ |
| `editor` | mod | src/core/mod.rs | 4 | ✓ |
| `ui` | mod | src/lib.rs | 2 | ✓ |
| `adapter` | mod | src/lib.rs | 4 | ✓ |
| `core` | mod | src/lib.rs | 6 | ✓ |
| `pal` | mod | src/lib.rs | 8 | ✓ |
| `gateway` | mod | src/lib.rs | 10 | ✓ |
| `shared` | mod | src/lib.rs | 12 | ✓ |
| `paragraph` | mod | src/pal/mod.rs | 2 | ✓ |
| `cursor` | mod | src/pal/mod.rs | 4 | ✓ |
| `media` | mod | src/pal/mod.rs | 6 | ✓ |
| `HIGHLIGHT_FADE_MS` | const | src/shared/durations.rs | 2 | ✓ |
| `UNDO_COALESCE_MS` | const | src/shared/durations.rs | 6 | ✓ |
| `MAX_UNDO_DEPTH` | const | src/shared/limits.rs | 2 | ✓ |
| `MAX_HEADING_LEVEL` | const | src/shared/limits.rs | 5 | ✓ |
| `sizes` | mod | src/shared/mod.rs | 2 | ✓ |
| `durations` | mod | src/shared/mod.rs | 4 | ✓ |
| `limits` | mod | src/shared/mod.rs | 6 | ✓ |
| `paths` | mod | src/shared/mod.rs | 8 | ✓ |
| `DOC_EXTENSION` | const | src/shared/paths.rs | 2 | ✓ |
| `VIDEO_EXTENSIONS` | const | src/shared/paths.rs | 5 | ✓ |
| `PARAGRAPH_CACHE_CAP` | const | src/shared/sizes.rs | 2 | ✓ |
| `THUMBNAIL_WIDTH` | const | src/shared/sizes.rs | 5 | ✓ |
| `THUMBNAIL_HEIGHT` | const | src/shared/sizes.rs | 8 | ✓ |
| `mcp` | mod | src/ui/mod.rs | 2 | ✓ |
| `gui` | mod | src/ui/mod.rs | 4 | ✓ |
| `AppWindow` | component | ui/main.slint | 4 | ⚠ |
