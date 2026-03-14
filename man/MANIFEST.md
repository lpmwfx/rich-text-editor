# Documentation Index

Generated: 2026-03-14T14:19:32.404939200+01:00  
Project: `project`  
Coverage: **37/38** items documented (**97%**) — ⚠ **1 undocumented**

## Files

| Source File | Items | Undocumented |
|---|---|---|
| [build.rs](man/build.md) | 0 | — |
| [src/adapter/editor_state_adp.rs](man/src/adapter/editor_state_adp.md) | 1 | ✓ |
| [src/adapter/mod.rs](man/src/adapter/mod.md) | 1 | ✓ |
| [src/core/document/frontmatter.rs](man/src/core/document/frontmatter.md) | 0 | — |
| [src/core/document/mod.rs](man/src/core/document/mod.md) | 7 | ✓ |
| [src/core/document/parser.rs](man/src/core/document/parser.md) | 0 | — |
| [src/core/document/serializer.rs](man/src/core/document/serializer.md) | 0 | — |
| [src/core/editor/commands.rs](man/src/core/editor/commands.md) | 0 | — |
| [src/core/editor/mod.rs](man/src/core/editor/mod.md) | 2 | ✓ |
| [src/core/editor/undo.rs](man/src/core/editor/undo.md) | 0 | — |
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
| `EditorState` | struct | src/adapter/editor_state_adp.rs | 4 | ✓ |
| `editor_state_adp` | mod | src/adapter/mod.rs | 2 | ✓ |
| `parser` | mod | src/core/document/mod.rs | 2 | ✓ |
| `serializer` | mod | src/core/document/mod.rs | 4 | ✓ |
| `frontmatter` | mod | src/core/document/mod.rs | 6 | ✓ |
| `Document` | struct | src/core/document/mod.rs | 10 | ✓ |
| `Frontmatter` | struct | src/core/document/mod.rs | 19 | ✓ |
| `Block` | enum | src/core/document/mod.rs | 26 | ✓ |
| `Inline` | enum | src/core/document/mod.rs | 67 | ✓ |
| `commands` | mod | src/core/editor/mod.rs | 2 | ✓ |
| `undo` | mod | src/core/editor/mod.rs | 4 | ✓ |
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
