---
name: Project Status
description: Current phase, goals, and architecture of the text editor
type: project
---

# Phase 2: Gateway & MCP

**Current work:** Implement file IO (gateway), MCP server integration, and 13 tools for Claude Desktop
**Status:** Development (phase 1 complete, 37 tests passing)

## Architecture
6-layer hexagonal MVVM:
- src/ui/_ui: Slint GUI + MCP server (dual surfaces)
- src/adapter/_adp: EditorState, ViewModel hub
- src/core/_core: Document AST, commands, undo
- src/pal/_pal: Skia/skparagraph rendering
- src/gateway/_gtw: File IO, media loading
- src/shared/_x: Constants, errors, types

## Key Rules
- All editing through UndoStack::apply() only
- Offsets are UTF-8 byte positions
- MCP mode: stdout JSON-RPC only, logging to stderr
- GUI mode: render via Skia, not Slint components
- No unwrap() in production code


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=4" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
