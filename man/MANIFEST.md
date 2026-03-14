# Documentation Index

Generated: 2026-03-14T13:16:28.960073700+01:00  
Project: `project`  
Coverage: **32/33** items documented (**96%**) — ⚠ **1 undocumented**

## Files

| Source File | Items | Undocumented |
|---|---|---|
| [build.rs](man/build.md) | 0 | — |
| [src/document/frontmatter.rs](man/src/document/frontmatter.md) | 0 | — |
| [src/document/mod.rs](man/src/document/mod.md) | 7 | ✓ |
| [src/document/parser.rs](man/src/document/parser.md) | 0 | — |
| [src/document/serializer.rs](man/src/document/serializer.md) | 0 | — |
| [src/editor/commands.rs](man/src/editor/commands.md) | 0 | — |
| [src/editor/mod.rs](man/src/editor/mod.md) | 3 | ✓ |
| [src/editor/undo.rs](man/src/editor/undo.md) | 0 | — |
| [src/lib.rs](man/src/lib.md) | 6 | ✓ |
| [src/main.rs](man/src/main.md) | 0 | — |
| [src/mcp/mod.rs](man/src/mcp/mod.md) | 0 | — |
| [src/media/mod.rs](man/src/media/mod.md) | 0 | — |
| [src/renderer/cursor.rs](man/src/renderer/cursor.md) | 0 | — |
| [src/renderer/media.rs](man/src/renderer/media.md) | 0 | — |
| [src/renderer/mod.rs](man/src/renderer/mod.md) | 3 | ✓ |
| [src/renderer/paragraph.rs](man/src/renderer/paragraph.md) | 0 | — |
| [src/state/durations.rs](man/src/state/durations.md) | 2 | ✓ |
| [src/state/limits.rs](man/src/state/limits.md) | 2 | ✓ |
| [src/state/mod.rs](man/src/state/mod.md) | 4 | ✓ |
| [src/state/paths.rs](man/src/state/paths.md) | 2 | ✓ |
| [src/state/sizes.rs](man/src/state/sizes.md) | 3 | ✓ |
| [ui/main.slint](man/ui/main.md) | 1 | ⚠ 1 |

## All Items

| Item | Kind | Source | Line | Documented |
|---|---|---|---|---|
| `parser` | mod | src/document/mod.rs | 2 | ✓ |
| `serializer` | mod | src/document/mod.rs | 4 | ✓ |
| `frontmatter` | mod | src/document/mod.rs | 6 | ✓ |
| `Document` | struct | src/document/mod.rs | 10 | ✓ |
| `Frontmatter` | struct | src/document/mod.rs | 19 | ✓ |
| `Block` | enum | src/document/mod.rs | 26 | ✓ |
| `Inline` | enum | src/document/mod.rs | 67 | ✓ |
| `commands` | mod | src/editor/mod.rs | 2 | ✓ |
| `undo` | mod | src/editor/mod.rs | 4 | ✓ |
| `EditorState` | struct | src/editor/mod.rs | 8 | ✓ |
| `document` | mod | src/lib.rs | 2 | ✓ |
| `editor` | mod | src/lib.rs | 4 | ✓ |
| `renderer` | mod | src/lib.rs | 6 | ✓ |
| `media` | mod | src/lib.rs | 8 | ✓ |
| `mcp` | mod | src/lib.rs | 10 | ✓ |
| `state` | mod | src/lib.rs | 12 | ✓ |
| `paragraph` | mod | src/renderer/mod.rs | 2 | ✓ |
| `cursor` | mod | src/renderer/mod.rs | 4 | ✓ |
| `media` | mod | src/renderer/mod.rs | 6 | ✓ |
| `HIGHLIGHT_FADE_MS` | const | src/state/durations.rs | 2 | ✓ |
| `UNDO_COALESCE_MS` | const | src/state/durations.rs | 6 | ✓ |
| `MAX_UNDO_DEPTH` | const | src/state/limits.rs | 2 | ✓ |
| `MAX_HEADING_LEVEL` | const | src/state/limits.rs | 5 | ✓ |
| `sizes` | mod | src/state/mod.rs | 2 | ✓ |
| `durations` | mod | src/state/mod.rs | 4 | ✓ |
| `limits` | mod | src/state/mod.rs | 6 | ✓ |
| `paths` | mod | src/state/mod.rs | 8 | ✓ |
| `DOC_EXTENSION` | const | src/state/paths.rs | 2 | ✓ |
| `VIDEO_EXTENSIONS` | const | src/state/paths.rs | 5 | ✓ |
| `PARAGRAPH_CACHE_CAP` | const | src/state/sizes.rs | 2 | ✓ |
| `THUMBNAIL_WIDTH` | const | src/state/sizes.rs | 5 | ✓ |
| `THUMBNAIL_HEIGHT` | const | src/state/sizes.rs | 8 | ✓ |
| `AppWindow` | component | ui/main.slint | 4 | ⚠ |
