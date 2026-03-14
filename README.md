# Rich Text Editor

WYSIWYG rich text editor built with Rust, Slint, and Skia/skparagraph.

## Features

- **Markdown backend** — all documents stored as standard Markdown
- **Skia text rendering** — skparagraph for high-quality text layout with inline formatting
- **Media embedding** — images and video via skparagraph placeholders
- **MCP server mode** — expose the editor as an MCP tool server for live manipulation by AI assistants
- **Undo/redo** — full command-pattern undo stack
- **Windows native** — Fluent 2 design via [SlintUITemplates](https://github.com/lpmwfx/SlintUITemplates)

## Usage

```bash
# GUI mode
cargo run

# MCP server mode (stdio transport, for Claude Desktop)
cargo run -- --mcp
```

## Architecture

- `src/document/` — Block/Inline AST, Markdown parser and serializer
- `src/editor/` — EditorState, cursor, selection, undo stack, commands
- `src/renderer/` — Skia/skparagraph rendering pipeline
- `src/media/` — Media asset manager with async loading
- `src/mcp/` — MCP server (tools + resources) via rmcp
- `ui/` — Slint UI components

## Dependencies

- [Slint](https://slint.dev) — UI framework
- [SlintUITemplates](https://github.com/lpmwfx/SlintUITemplates) — desktop shell, theme, tokens
- [skia-safe](https://crates.io/crates/skia-safe) — Skia bindings with textlayout
- [rmcp](https://crates.io/crates/rmcp) — MCP server framework
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) — Markdown parsing

## License

EUPL-1.2
